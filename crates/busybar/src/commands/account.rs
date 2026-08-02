use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    /// Show the linked account
    Info,

    /// Show the cloud connection status
    Status,

    /// Show the MQTT backend configuration
    Backend,
}

impl AccountCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
