use std::path::PathBuf;

use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
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
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
