use std::borrow::Cow;
use std::path::PathBuf;

use busylib::TungsteniteWsTransport;
use clap::Subcommand;

use std::path::Path;

use crate::cli::Context;
use crate::error::Result;
use crate::io::Io;
use busylib::proto::bsb_frame::{Frame as StreamedFrame, Screen};
use busylib::proto::bsb_state::State;
use busylib::proto::bsb_state::state_update::State as Update;

use crate::reporter::{FramePayload, StreamingScreenEvent, StreamingStatusEvent};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use busybar_render::{ImageFormat, Raster};

use crate::types::frame::{Frame, screen_name};
use crate::types::image_format_arg::ImageFormatArg;
use crate::types::output_format::OutputFormatArg;
use crate::types::screen_arg::ScreenArg;

#[derive(Debug, Subcommand)]
pub enum StreamingCommand {
    /// Capture a single frame of a screen
    Screen {
        /// Screen to capture
        #[arg(value_enum, default_value_t = ScreenArg::Front)]
        screen: ScreenArg,

        /// Write the frame to disk instead of stdout.
        ///
        /// If the output path ends in a .bmp, .jpg or .png extension, then the raw frame
        /// is converted to that format respectively.
        #[arg(long, short = 'O', value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Stream device status over a WebSocket until interrupted with ctrl-c
    StatusWs {
        /// Decode streamed frames and write them to this folder, see --image-format
        #[arg(long, value_name = "DIR")]
        frame_dir: Option<PathBuf>,

        /// Do not space the pixels of front frames out on the black raster of the matrix
        #[arg(long)]
        no_image_raster: bool,
    },
}

impl StreamingCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            StreamingCommand::Screen {
                screen: screen_arg,
                output,
            } => screen(context, screen_arg, output.as_deref()).await,
            StreamingCommand::StatusWs {
                frame_dir,
                no_image_raster,
            } => {
                let raster = (!no_image_raster).then(Raster::default);
                status_ws(context, frame_dir.as_deref(), raster).await
            }
        }
    }
}

async fn screen(context: &Context, screen: ScreenArg, output: Option<&Path>) -> Result<()> {
    let body = context.client.streaming().screen(screen.into()).await?;

    let frame = match context.image_format.image_format() {
        Some(format) => Cow::Owned(Frame::decode(screen, &body)?.encode(format)?),
        None => Cow::Borrowed(body.as_ref()),
    };

    if let Some(payload) = Io::output_binary_data(context.output_format, &frame, output)? {
        context
            .reporter
            .report(StreamingScreenEvent::new(payload))?;
    }

    Ok(())
}

async fn status_ws(
    context: &Context,
    frame_dir: Option<&Path>,
    raster: Option<Raster>,
) -> Result<()> {
    let transport = TungsteniteWsTransport::new();
    let mut stream = context.client.streaming().status_ws(&transport).await?;

    if let Some(directory) = frame_dir {
        Io::create_dir(directory)?;
    }

    let mut sequence = 0u64;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => break,
            message = stream.next() => match message {
                Some(message) => {
                    for event in events(context, &mut sequence, message?, frame_dir, raster) {
                        context.reporter.report(event)?;
                    }
                }
                None => break,
            },
        }
    }

    stream.close().await?;

    Ok(())
}

fn events(
    context: &Context,
    sequence: &mut u64,
    state: State,
    frame_dir: Option<&Path>,
    raster: Option<Raster>,
) -> Vec<StreamingStatusEvent> {
    let error = StreamingStatusEvent::error_of(&state);
    let timestamp = state.timestamp;

    if state.updates.is_empty() {
        *sequence += 1;
        return vec![StreamingStatusEvent::heartbeat(*sequence, timestamp, error)];
    }

    let mut events = Vec::with_capacity(state.updates.len());

    for update in state.updates {
        let Some(update) = update.state else {
            continue;
        };

        *sequence += 1;

        let event = match update {
            Update::Frame(frame) => {
                let payload = frame_payload(context, *sequence, &frame, frame_dir, raster);
                StreamingStatusEvent::frame(*sequence, timestamp, error, payload)
            }
            update => StreamingStatusEvent::update(*sequence, timestamp, error, update),
        };

        events.push(event);
    }

    events
}

fn frame_payload(
    context: &Context,
    sequence: u64,
    frame: &StreamedFrame,
    frame_dir: Option<&Path>,
    raster: Option<Raster>,
) -> FramePayload {
    let selected = context.image_format;
    let re_encode = selected.image_format();
    let inline = matches!(context.output_format, OutputFormatArg::Json);

    let raster = raster.filter(|_| frame.screen() == Screen::Front);

    let mut reason = None;

    let decoded = match re_encode {
        Some(_) if inline || frame_dir.is_some() => {
            match Frame::from_streamed(frame).and_then(|image| match raster {
                Some(raster) => image.with_raster(raster),
                None => Ok(image),
            }) {
                Ok(image) => Some(image),
                Err(error) => {
                    reason = Some(error.to_string());
                    None
                }
            }
        }
        _ => None,
    };

    let path = match frame_dir {
        Some(directory) if re_encode.is_none() || decoded.is_some() => {
            let name = format!("{}-{sequence:06}.{selected}", screen_name(frame.screen()));

            match write_frame(&directory.join(name), frame, decoded.as_ref(), re_encode) {
                Ok(path) => Some(path),
                Err(error) => {
                    reason.get_or_insert(error.to_string());
                    None
                }
            }
        }
        _ => None,
    };

    let encoded = match (&decoded, re_encode) {
        (Some(image), Some(format)) if inline => match image.encode_base64(format) {
            Ok(encoded) => Some(encoded),
            Err(error) => {
                reason.get_or_insert(error.to_string());
                None
            }
        },
        _ => None,
    };

    let image = inline.then(|| match encoded {
        Some(encoded) => (selected.to_string(), encoded),
        None => (
            ImageFormatArg::Raw.to_string(),
            STANDARD.encode(&frame.data),
        ),
    });

    FramePayload::new(frame, image, path, reason)
}

fn write_frame(
    path: &Path,
    frame: &StreamedFrame,
    decoded: Option<&Frame>,
    format: Option<ImageFormat>,
) -> Result<String> {
    let bytes = match (decoded, format) {
        (Some(image), Some(format)) => Cow::Owned(image.encode(format)?),
        _ => Cow::Borrowed(frame.data.as_slice()),
    };

    Io::write_bytes(path, &bytes)?;

    Ok(path.display().to_string())
}
