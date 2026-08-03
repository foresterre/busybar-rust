use std::path::{Path, PathBuf};

use busylib::model::assets::{DisplayElements, PlayAudio};
use busylib::types::app_name::AppName;
use busylib::types::asset_name::AssetName;
use busylib::types::asset_path::AssetPath;
use busylib::types::stock_path::StockPath;
use clap::{ArgGroup, Subcommand};

use crate::cli::Context;
use crate::error::{CliError, Result};
use crate::io::Io;
use crate::reporter::OkEvent;

#[derive(Debug, Subcommand)]
pub enum AssetsCommand {
    /// Upload a file into an application's assets
    Upload {
        /// Application name
        #[arg(long, short = 'a', value_name = "NAME")]
        app: AppName,

        /// Asset file name on the device
        #[arg(long, short = 'f', value_name = "NAME")]
        file: AssetName,

        /// Local file to upload
        #[arg(value_name = "FILE")]
        source: PathBuf,
    },

    /// Delete every asset of an application
    Delete {
        /// Application name
        #[arg(long, short = 'a', value_name = "NAME")]
        app: AppName,
    },

    /// Play a sound from an application's assets
    #[command(group = ArgGroup::new("audio-source").required(true).args(["path", "stock"]))]
    Play {
        /// Application name
        #[arg(long, short = 'a', value_name = "NAME")]
        app: AppName,

        /// Sound in the application's assets
        #[arg(long, value_name = "PATH")]
        path: Option<AssetPath>,

        /// Stock sound
        #[arg(long, value_name = "PATH")]
        stock: Option<StockPath>,
    },

    /// Stop the playing sound
    Stop,

    /// Draw elements from a JSON payload
    Draw {
        /// JSON draw request, or - for stdin
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Clear drawn elements
    Clear {
        /// Only clear the elements of this application
        #[arg(long, short = 'a', value_name = "NAME")]
        app: Option<AppName>,
    },
}

impl AssetsCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            AssetsCommand::Upload { app, file, source } => {
                upload(context, app, file, &source).await
            }
            AssetsCommand::Delete { app } => delete(context, app).await,
            AssetsCommand::Play { app, path, stock } => play(context, app, path, stock).await,
            AssetsCommand::Stop => stop(context).await,
            AssetsCommand::Draw { file } => draw(context, &file).await,
            AssetsCommand::Clear { app } => clear(context, app).await,
        }
    }
}

async fn upload(context: &Context, app: AppName, file: AssetName, source: &Path) -> Result<()> {
    let data = Io::read_bytes(source)?;

    context.client.assets().upload(app, file, data).await?;

    context.reporter.report(OkEvent::new("assets upload"))?;

    Ok(())
}

async fn delete(context: &Context, app: AppName) -> Result<()> {
    context.client.assets().delete(app).await?;

    context.reporter.report(OkEvent::new("assets delete"))?;

    Ok(())
}

async fn play(
    context: &Context,
    app: AppName,
    path: Option<AssetPath>,
    stock: Option<StockPath>,
) -> Result<()> {
    let audio = match (path, stock) {
        (Some(path), _) => PlayAudio::asset(app, path)?,
        (None, Some(stock)) => PlayAudio::stock(app, stock)?,
        (None, None) => return Err(CliError::Usage("either --path or --stock is required")),
    };

    context.client.assets().play(&audio).await?;

    context.reporter.report(OkEvent::new("assets play"))?;

    Ok(())
}

async fn stop(context: &Context) -> Result<()> {
    context.client.assets().stop().await?;

    context.reporter.report(OkEvent::new("assets stop"))?;

    Ok(())
}

async fn draw(context: &Context, file: &Path) -> Result<()> {
    let elements: DisplayElements = Io::read_json(file)?;

    context.client.assets().draw(&elements).await?;

    context.reporter.report(OkEvent::new("assets draw"))?;

    Ok(())
}

async fn clear(context: &Context, app: Option<AppName>) -> Result<()> {
    context.client.assets().clear(app).await?;

    context.reporter.report(OkEvent::new("assets clear"))?;

    Ok(())
}
