use busylib::types::log_name::LogName;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::{
    SystemLogDumpEvent, SystemStatusDeviceEvent, SystemStatusEvent, SystemStatusFirmwareEvent,
    SystemStatusPowerEvent, SystemStatusSystemEvent, SystemTransportEvent, SystemVersionEvent,
};

#[derive(Debug, Subcommand)]
pub enum SystemCommand {
    /// Show the HTTP API version
    Version,

    /// Show how the device is connected
    Transport,

    /// Show device, firmware, system and power information
    Status,

    /// Show device information
    StatusDevice,

    /// Show firmware information
    StatusFirmware,

    /// Show system information
    StatusSystem,

    /// Show power and battery information
    StatusPower,

    /// Write the in-memory log to a file
    LogDump {
        /// Destination file name without extension
        #[arg(long, short = 'f', value_name = "NAME")]
        filename: Option<LogName>,
    },
}

impl SystemCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            SystemCommand::Version => version(context).await,
            SystemCommand::Transport => transport(context).await,
            SystemCommand::Status => status(context).await,
            SystemCommand::StatusDevice => status_device(context).await,
            SystemCommand::StatusFirmware => status_firmware(context).await,
            SystemCommand::StatusSystem => status_system(context).await,
            SystemCommand::StatusPower => status_power(context).await,
            SystemCommand::LogDump { filename } => log_dump(context, filename).await,
        }
    }
}

async fn version(context: &Context) -> Result<()> {
    let version = context.client.system().version().await?;

    context.reporter.report(SystemVersionEvent::new(version))?;

    Ok(())
}

async fn transport(context: &Context) -> Result<()> {
    let transport = context.client.system().transport().await?;

    context
        .reporter
        .report(SystemTransportEvent::new(transport))?;

    Ok(())
}

async fn status(context: &Context) -> Result<()> {
    let status = context.client.system().status().await?;

    context.reporter.report(SystemStatusEvent::new(status))?;

    Ok(())
}

async fn status_device(context: &Context) -> Result<()> {
    let device = context.client.system().status_device().await?;

    context
        .reporter
        .report(SystemStatusDeviceEvent::new(device))?;

    Ok(())
}

async fn status_firmware(context: &Context) -> Result<()> {
    let firmware = context.client.system().status_firmware().await?;

    context
        .reporter
        .report(SystemStatusFirmwareEvent::new(firmware))?;

    Ok(())
}

async fn status_system(context: &Context) -> Result<()> {
    let system = context.client.system().status_system().await?;

    context
        .reporter
        .report(SystemStatusSystemEvent::new(system))?;

    Ok(())
}

async fn log_dump(context: &Context, filename: Option<LogName>) -> Result<()> {
    let path = context.client.system().log_dump(filename).await?;

    context.reporter.report(SystemLogDumpEvent::new(path))?;

    Ok(())
}

async fn status_power(context: &Context) -> Result<()> {
    let power = context.client.system().status_power().await?;

    context
        .reporter
        .report(SystemStatusPowerEvent::new(power))?;

    Ok(())
}
