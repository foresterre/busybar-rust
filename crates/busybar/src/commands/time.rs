use busylib::types::timestamp::Timestamp;
use busylib::types::timezone_name::TimezoneName;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::{OkEvent, TimeNowEvent, TimeTimezoneEvent, TimeTzlistEvent};

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
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            TimeCommand::Now => now(context).await,
            TimeCommand::SetTimestamp { timestamp } => set_timestamp(context, timestamp).await,
            TimeCommand::Timezone => timezone(context).await,
            TimeCommand::SetTimezone { name } => set_timezone(context, name).await,
            TimeCommand::Tzlist => tzlist(context).await,
        }
    }
}

async fn now(context: &Context) -> Result<()> {
    let now = context.client.time().now().await?;

    context.reporter.report(TimeNowEvent::new(now))?;

    Ok(())
}

async fn set_timestamp(context: &Context, timestamp: Timestamp) -> Result<()> {
    context.client.time().set_timestamp(timestamp).await?;

    context
        .reporter
        .report(OkEvent::new("time set-timestamp"))?;

    Ok(())
}

async fn timezone(context: &Context) -> Result<()> {
    let timezone = context.client.time().timezone().await?;

    context.reporter.report(TimeTimezoneEvent::new(timezone))?;

    Ok(())
}

async fn set_timezone(context: &Context, name: TimezoneName) -> Result<()> {
    context.client.time().set_timezone(name).await?;

    context.reporter.report(OkEvent::new("time set-timezone"))?;

    Ok(())
}

async fn tzlist(context: &Context) -> Result<()> {
    let list = context.client.time().tzlist().await?;

    context.reporter.report(TimeTzlistEvent::new(list))?;

    Ok(())
}
