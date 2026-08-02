use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::values::{SwitchStartupArg, SwitchStateArg};

#[derive(Debug, Subcommand)]
pub enum SmartHomeCommand {
    /// Show the smart home pairing status
    Pairing,

    /// Start pairing with a smart home
    StartPairing,

    /// Erase every smart home link
    ErasePairings,

    /// Show the state of the emulated switch
    Switch,

    /// Set the state of the emulated switch
    SetSwitch {
        /// Switch state
        #[arg(value_enum, value_name = "STATE")]
        state: SwitchStateArg,

        /// State of the switch after a restart
        #[arg(long, value_enum, value_name = "STATE")]
        startup: Option<SwitchStartupArg>,
    },
}

impl SmartHomeCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
