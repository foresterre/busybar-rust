use std::path::PathBuf;

use clap::Subcommand;

use std::path::Path;

use crate::cli::Context;
use crate::error::Result;
use crate::io::Io;
use crate::reporter::StreamingScreenEvent;
use crate::values::ScreenArg;

#[derive(Debug, Subcommand)]
pub enum StreamingCommand {
    /// Capture a single frame of a screen
    Screen {
        /// Screen to capture
        #[arg(value_enum, default_value_t = ScreenArg::Front)]
        screen: ScreenArg,

        /// Write the frame here instead of stdout
        #[arg(long, short = 'O', value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// Stream device status over a WebSocket
    StatusWs,
}

impl StreamingCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            StreamingCommand::Screen { screen, output } => {
                self::screen(context, screen, output.as_deref()).await
            }
            // `/status/ws` upgrades to a WebSocket, which `HttpTransport` cannot carry.
            StreamingCommand::StatusWs => todo!(),
        }
    }
}

async fn screen(context: &Context, screen: ScreenArg, output: Option<&Path>) -> Result<()> {
    let frame = context.client.streaming().screen(screen.into()).await?;

    if let Some(payload) = Io::output_binary_data(context.output_format, &frame, output)? {
        context
            .reporter
            .report(StreamingScreenEvent::new(payload))?;
    }

    Ok(())
}
