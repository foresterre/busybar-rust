use std::path::{Path, PathBuf};

use busylib::model::busy::{BusyProfile, BusySnapshot};
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::io::Io;
use crate::reporter::{BusyProfileEvent, BusySnapshotEvent, OkEvent};
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
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            BusyCommand::Snapshot => snapshot(context).await,
            BusyCommand::SetSnapshot { file } => set_snapshot(context, &file).await,
            BusyCommand::Profile { slot } => profile(context, slot).await,
            BusyCommand::SetProfile { slot, file } => set_profile(context, slot, &file).await,
        }
    }
}

async fn snapshot(context: &Context) -> Result<()> {
    let snapshot = context.client.busy().snapshot().await?;

    context.reporter.report(BusySnapshotEvent::new(snapshot))?;

    Ok(())
}

async fn set_snapshot(context: &Context, file: &Path) -> Result<()> {
    let snapshot: BusySnapshot = Io::read_json(file)?;

    context.client.busy().set_snapshot(&snapshot).await?;

    context.reporter.report(OkEvent::new("busy set-snapshot"))?;

    Ok(())
}

async fn profile(context: &Context, slot: SlotArg) -> Result<()> {
    let profile = context.client.busy().profile(slot.into()).await?;

    context.reporter.report(BusyProfileEvent::new(profile))?;

    Ok(())
}

async fn set_profile(context: &Context, slot: SlotArg, file: &Path) -> Result<()> {
    let profile: BusyProfile = Io::read_json(file)?;

    context
        .client
        .busy()
        .set_profile(slot.into(), &profile)
        .await?;

    context.reporter.report(OkEvent::new("busy set-profile"))?;

    Ok(())
}
