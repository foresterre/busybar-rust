use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum BleCommand {
    /// Enable BLE and start advertising
    Enable,

    /// Stop advertising
    Disable,

    /// Remove the pairing with the previous device
    RemovePairing,

    /// Show the BLE status
    Status,
}

impl BleCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
