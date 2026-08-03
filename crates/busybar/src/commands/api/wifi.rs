use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::WifiStatusEvent;

#[derive(Debug, Subcommand)]
pub enum WifiCommand {
    /// Show the Wi-Fi status
    Status,
}

impl WifiCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            WifiCommand::Status => status(context).await,
        }
    }
}

async fn status(context: &Context) -> Result<()> {
    let status = context.client.wifi().status().await?;

    context.reporter.report(WifiStatusEvent::new(status))?;

    Ok(())
}
