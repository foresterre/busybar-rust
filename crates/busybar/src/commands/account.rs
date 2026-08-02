use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::{AccountBackendEvent, AccountInfoEvent, AccountStatusEvent};

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
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            AccountCommand::Info => info(context).await,
            AccountCommand::Status => status(context).await,
            AccountCommand::Backend => backend(context).await,
        }
    }
}

async fn info(context: &Context) -> Result<()> {
    let info = context.client.account().info().await?;

    context.reporter.report(AccountInfoEvent::new(info))?;

    Ok(())
}

async fn status(context: &Context) -> Result<()> {
    let status = context.client.account().status().await?;

    context.reporter.report(AccountStatusEvent::new(status))?;

    Ok(())
}

async fn backend(context: &Context) -> Result<()> {
    let backend = context.client.account().backend().await?;

    context.reporter.report(AccountBackendEvent::new(backend))?;

    Ok(())
}
