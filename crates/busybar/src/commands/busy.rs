use std::path::PathBuf;

use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::values::SlotArg;

#[derive(Debug, Subcommand)]
pub enum BusyCommand {
    /// Show the current timer snapshot
    Snapshot,

    /// Run the timer from a snapshot
    SetSnapshot {
        /// JSON snapshot, or - for stdin
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Show the timer profile of a slot
    Profile {
        /// Profile slot
        #[arg(value_enum, value_name = "SLOT")]
        slot: SlotArg,
    },

    /// Replace the timer profile of a slot
    SetProfile {
        /// Profile slot
        #[arg(value_enum, value_name = "SLOT")]
        slot: SlotArg,

        /// JSON profile, or - for stdin
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },
}

impl BusyCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
