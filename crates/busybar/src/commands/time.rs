use busylib::types::timestamp::Timestamp;
use busylib::types::timezone_name::TimezoneName;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum TimeCommand {
    /// Show the device clock
    Now,

    /// Set the device clock
    SetTimestamp {
        /// ISO 8601 timestamp with time zone
        #[arg(value_name = "TIMESTAMP")]
        timestamp: Timestamp,
    },

    /// Show the time zone
    Timezone,

    /// Set the time zone
    SetTimezone {
        /// Time zone name
        #[arg(value_name = "NAME")]
        name: TimezoneName,
    },

    /// List the supported time zones
    Tzlist,
}

impl TimeCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
