use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum WifiCommand {
    /// Show the Wi-Fi status
    Status,
}

impl WifiCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
