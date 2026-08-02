use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::{BleStatusEvent, OkEvent};

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
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            BleCommand::Enable => enable(context).await,
            BleCommand::Disable => disable(context).await,
            BleCommand::RemovePairing => remove_pairing(context).await,
            BleCommand::Status => status(context).await,
        }
    }
}

async fn enable(context: &Context) -> Result<()> {
    context.client.ble().enable().await?;

    context.reporter.report(OkEvent::new("ble enable"))?;

    Ok(())
}

async fn disable(context: &Context) -> Result<()> {
    context.client.ble().disable().await?;

    context.reporter.report(OkEvent::new("ble disable"))?;

    Ok(())
}

async fn remove_pairing(context: &Context) -> Result<()> {
    context.client.ble().remove_pairing().await?;

    context
        .reporter
        .report(OkEvent::new("ble remove-pairing"))?;

    Ok(())
}

async fn status(context: &Context) -> Result<()> {
    let status = context.client.ble().status().await?;

    context.reporter.report(BleStatusEvent::new(status))?;

    Ok(())
}
