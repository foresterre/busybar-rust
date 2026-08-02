use busylib::types::log_name::LogName;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum SystemCommand {
    /// Show the HTTP API version
    Version,

    /// Show how the device is connected
    Transport,

    /// Show device, firmware, system and power information
    Status,

    /// Show device information
    Device,

    /// Show firmware information
    Firmware,

    /// Show system information
    Info,

    /// Show power and battery information
    Power,

    /// Write the in-memory log to a file
    DumpLog {
        /// Destination file name without extension
        #[arg(long, short = 'f', value_name = "NAME")]
        filename: Option<LogName>,
    },
}

impl SystemCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
