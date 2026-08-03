use std::borrow::Cow;
use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use busybar_render::{ImageFormat, Raster};
use busylib::TungsteniteWsTransport;
use busylib::proto::bsb_frame::{Frame as StreamedFrame, Screen};
use busylib::proto::bsb_state::State;
use busylib::proto::bsb_state::state_update::State as Update;

use crate::cli::Context;
use crate::error::Result;
use crate::io::Io;
use crate::reporter::{FramePayload, StreamingStatusEvent};
use crate::types::frame::{Frame, screen_name};
use crate::types::image_format_arg::ImageFormatArg;
use crate::types::output_format::OutputFormatArg;
use crate::types::screen_arg::ScreenArg;

pub async fn run(
    context: &Context,
    directory: &Path,
    screen: ScreenArg,
    no_image_raster: bool,
) -> Result<()> {
    let screen = Screen::from(screen);
    let raster = (!no_image_raster && screen == Screen::Front).then(Raster::default);

    Io::create_dir(directory)?;

    let transport = TungsteniteWsTransport::new();
    let mut stream = context.client.streaming().status_ws(&transport).await?;

    let mut sequence = 0u64;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => break,
            message = stream.next() => match message {
                Some(message) => {
                    // TODO(foresterre): the back screen doesn't seem to send events over the screen, so we be might only
                    //                   be able to capture indivial frames via `api streaming screen { screen = back } `
                    for event in events(context, &mut sequence, message?, directory, screen, raster) {
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
    directory: &Path,
    screen: Screen,
    raster: Option<Raster>,
) -> Vec<StreamingStatusEvent> {
    let error = StreamingStatusEvent::error_of(&state);
    let timestamp = state.timestamp;

    let mut events = Vec::new();

    for update in state.updates {
        let Some(Update::Frame(frame)) = update.state else {
            continue;
        };

        if frame.screen() != screen {
            continue;
        }

        *sequence += 1;

        let payload = capture(context, *sequence, &frame, directory, raster);
        events.push(StreamingStatusEvent::frame(
            *sequence, timestamp, error, payload,
        ));
    }

    events
}

fn capture(
    context: &Context,
    sequence: u64,
    frame: &StreamedFrame,
    directory: &Path,
    raster: Option<Raster>,
) -> FramePayload {
    let selected = context.image_format;
    let re_encode = selected.image_format();
    let inline = matches!(context.output_format, OutputFormatArg::Json);

    let mut reason = None;

    let decoded = match re_encode {
        Some(_) => match Frame::from_streamed(frame).and_then(|image| match raster {
            Some(raster) => image.with_raster(raster),
            None => Ok(image),
        }) {
            Ok(image) => Some(image),
            Err(error) => {
                reason = Some(error.to_string());
                None
            }
        },
        None => None,
    };

    let path = if re_encode.is_none() || decoded.is_some() {
        let name = format!("{}-{sequence:06}.{selected}", screen_name(frame.screen()));

        match write_frame(&directory.join(name), frame, decoded.as_ref(), re_encode) {
            Ok(path) => Some(path),
            Err(error) => {
                reason.get_or_insert(error.to_string());
                None
            }
        }
    } else {
        None
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
