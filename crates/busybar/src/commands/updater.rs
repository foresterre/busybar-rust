use std::path::{Path, PathBuf};

use busylib::model::updater::AutoupdateSettings;
use busylib::types::time_of_day::TimeOfDay;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::{CliError, Result};
use crate::io::Io;
use crate::reporter::{OkEvent, UpdaterAutoupdateEvent, UpdaterChangelogEvent, UpdaterStatusEvent};

#[derive(Debug, Subcommand)]
pub enum UpdaterCommand {
    /// Upload a firmware package and start the update
    Update {
        /// Firmware TAR package
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Start a check for available firmware
    Check,

    /// Show the update and check status
    Status,

    /// Show the changelog of a firmware version
    Changelog {
        /// Firmware version
        #[arg(value_name = "VERSION")]
        version: String,
    },

    /// Install a firmware version from the cloud
    Install {
        /// Firmware version
        #[arg(value_name = "VERSION")]
        version: String,
    },

    /// Abort the running firmware download
    AbortDownload,

    /// Show the autoupdate settings
    Autoupdate,

    /// Change the autoupdate settings
    SetAutoupdate {
        /// Turn autoupdate on
        #[arg(long)]
        enable: bool,

        /// Turn autoupdate off
        #[arg(long, conflicts_with = "enable")]
        disable: bool,

        /// Start of the autoupdate window
        #[arg(long, value_name = "HH:MM")]
        start: Option<TimeOfDay>,

        /// End of the autoupdate window
        #[arg(long, value_name = "HH:MM")]
        end: Option<TimeOfDay>,
    },
}

impl UpdaterCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            UpdaterCommand::Update { file } => update(context, &file).await,
            UpdaterCommand::Check => check(context).await,
            UpdaterCommand::Status => status(context).await,
            UpdaterCommand::Changelog { version } => changelog(context, &version).await,
            UpdaterCommand::Install { version } => install(context, &version).await,
            UpdaterCommand::AbortDownload => abort_download(context).await,
            UpdaterCommand::Autoupdate => autoupdate(context).await,
            UpdaterCommand::SetAutoupdate {
                enable,
                disable,
                start,
                end,
            } => set_autoupdate(context, enable, disable, start, end).await,
        }
    }
}

async fn update(context: &Context, file: &Path) -> Result<()> {
    let package = Io::read_bytes(file)?;

    context.client.updater().update(package).await?;

    context.reporter.report(OkEvent::new("updater update"))?;

    Ok(())
}

async fn check(context: &Context) -> Result<()> {
    context.client.updater().check().await?;

    context.reporter.report(OkEvent::new("updater check"))?;

    Ok(())
}

async fn status(context: &Context) -> Result<()> {
    let status = context.client.updater().status().await?;

    context.reporter.report(UpdaterStatusEvent::new(status))?;

    Ok(())
}

async fn changelog(context: &Context, version: &str) -> Result<()> {
    let changelog = context.client.updater().changelog(version).await?;

    context
        .reporter
        .report(UpdaterChangelogEvent::new(changelog))?;

    Ok(())
}

async fn install(context: &Context, version: &str) -> Result<()> {
    context.client.updater().install(version).await?;

    context.reporter.report(OkEvent::new("updater install"))?;

    Ok(())
}

async fn abort_download(context: &Context) -> Result<()> {
    context.client.updater().abort_download().await?;

    context
        .reporter
        .report(OkEvent::new("updater abort-download"))?;

    Ok(())
}

async fn autoupdate(context: &Context) -> Result<()> {
    let settings = context.client.updater().autoupdate().await?;

    context
        .reporter
        .report(UpdaterAutoupdateEvent::new(settings))?;

    Ok(())
}

async fn set_autoupdate(
    context: &Context,
    enable: bool,
    disable: bool,
    start: Option<TimeOfDay>,
    end: Option<TimeOfDay>,
) -> Result<()> {
    let mut settings = AutoupdateSettings::new();

    if enable || disable {
        settings = settings.enabled(enable);
    }

    match (start, end) {
        (Some(start), Some(end)) => settings = settings.window(start, end),
        (Some(_), None) | (None, Some(_)) => {
            return Err(CliError::Usage("--start and --end go together"));
        }
        (None, None) => {}
    }

    context.client.updater().set_autoupdate(&settings).await?;

    context
        .reporter
        .report(OkEvent::new("updater set-autoupdate"))?;

    Ok(())
}
