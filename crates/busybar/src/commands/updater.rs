use std::path::PathBuf;

use busylib::types::time_of_day::TimeOfDay;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum UpdaterCommand {
    /// Upload a firmware package and start the update
    Upload {
        /// Firmware TAR package
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Start a check for available firmware
    Check,

    /// Show the update and check status
    Status,

    /// Show the changelog of a firmware version
    Changelog {
        /// Firmware version
        #[arg(value_name = "VERSION")]
        version: String,
    },

    /// Install a firmware version from the cloud
    Install {
        /// Firmware version
        #[arg(value_name = "VERSION")]
        version: String,
    },

    /// Abort the running firmware download
    AbortDownload,

    /// Show the autoupdate settings
    Autoupdate,

    /// Change the autoupdate settings
    SetAutoupdate {
        /// Turn autoupdate on
        #[arg(long)]
        enable: bool,

        /// Turn autoupdate off
        #[arg(long, conflicts_with = "enable")]
        disable: bool,

        /// Start of the autoupdate window
        #[arg(long, value_name = "HH:MM")]
        start: Option<TimeOfDay>,

        /// End of the autoupdate window
        #[arg(long, value_name = "HH:MM")]
        end: Option<TimeOfDay>,
    },
}

impl UpdaterCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
