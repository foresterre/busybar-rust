use std::io::Write;

use busybar_render::{MIRROR_RASTER, Mirror, Raster};
use busylib::TungsteniteWsTransport;
use busylib::proto::bsb_frame::{Frame as StreamedFrame, Screen};
use busylib::proto::bsb_state::State;
use busylib::proto::bsb_state::state_update::State as Update;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::UnsupportedEvent;
use crate::types::frame::{Frame, FrameError};
use crate::types::screen_arg::ScreenArg;

pub async fn run(context: &Context, screen: ScreenArg, no_screen_raster: bool) -> Result<()> {
    if let Some(event) = UnsupportedEvent::output_format(
        "mirror",
        "can't mirror frames from the device visually with json output =(",
        context.output_format,
    ) {
        return context.reporter.report(event).map_err(Into::into);
    }

    let screen = Screen::from(screen);
    let raster = (!no_screen_raster && screen == Screen::Front).then_some(MIRROR_RASTER);

    let transport = TungsteniteWsTransport::new();
    let mut stream = context.client.streaming().status_ws(&transport).await?;

    let mut mirror = Mirror::new(std::io::stdout().lock());
    mirror.enter()?;

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => break,
            message = stream.next() => match message {
                Some(message) => self::draw(&mut mirror, message?, screen, raster)?,
                None => break,
            },
        }
    }

    mirror.leave()?;
    stream.close().await?;

    Ok(())
}

fn draw<W: Write>(
    mirror: &mut Mirror<W>,
    state: State,
    screen: Screen,
    raster: Option<Raster>,
) -> Result<()> {
    for update in state.updates {
        let Some(Update::Frame(frame)) = update.state else {
            continue;
        };

        if frame.screen() != screen {
            continue;
        }

        match self::decode(&frame, raster) {
            Ok(frame) => mirror.draw(frame.image())?,
            Err(error) => mirror.notice(&error.to_string())?,
        }
    }

    Ok(())
}

fn decode(frame: &StreamedFrame, raster: Option<Raster>) -> std::result::Result<Frame, FrameError> {
    let frame = Frame::from_streamed(frame)?;

    match raster {
        Some(raster) => frame.with_raster(raster),
        None => Ok(frame),
    }
}
