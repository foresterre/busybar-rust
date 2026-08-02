use std::path::{Path, PathBuf};

use busylib::types::storage_path::StoragePath;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::io::Io;
use crate::reporter::{OkEvent, StorageListEvent, StorageReadEvent, StorageStatusEvent};

#[derive(Debug, Subcommand)]
pub enum StorageCommand {
    /// Upload a file to the internal storage
    Write {
        /// Destination below /ext
        #[arg(value_name = "PATH")]
        path: StoragePath,

        /// Local file to upload, or - for stdin
        #[arg(long, short = 'f', value_name = "FILE")]
        file: PathBuf,
    },

    /// Download a file from the internal storage
    Read {
        /// File to download
        #[arg(value_name = "PATH")]
        path: StoragePath,

        /// Write the file here instead of stdout
        #[arg(long, short = 'O', value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// List a directory on the internal storage
    List {
        /// Directory to list
        #[arg(value_name = "PATH", default_value = "/ext")]
        path: StoragePath,
    },

    /// Remove a file from the internal storage
    Remove {
        /// File to remove
        #[arg(value_name = "PATH")]
        path: StoragePath,
    },

    /// Create a directory on the internal storage
    Mkdir {
        /// Directory to create
        #[arg(value_name = "PATH")]
        path: StoragePath,
    },

    /// Move a file on the internal storage
    Rename {
        /// Current location
        #[arg(value_name = "PATH")]
        path: StoragePath,

        /// New location
        #[arg(value_name = "NEW_PATH")]
        new_path: StoragePath,
    },

    /// Show the storage usage
    Status,
}

impl StorageCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            StorageCommand::Write { path, file } => write(context, path, &file).await,
            StorageCommand::Read { path, output } => read(context, path, output.as_deref()).await,
            StorageCommand::List { path } => list(context, path).await,
            StorageCommand::Remove { path } => remove(context, path).await,
            StorageCommand::Mkdir { path } => mkdir(context, path).await,
            StorageCommand::Rename { path, new_path } => rename(context, path, new_path).await,
            StorageCommand::Status => status(context).await,
        }
    }
}

async fn write(context: &Context, path: StoragePath, file: &Path) -> Result<()> {
    let data = Io::read_bytes(file)?;

    context.client.storage().write(path, data).await?;

    context.reporter.report(OkEvent::new("storage write"))?;

    Ok(())
}

async fn read(context: &Context, path: StoragePath, output: Option<&Path>) -> Result<()> {
    let data = context.client.storage().read(path).await?;

    if let Some(payload) = Io::output_binary_data(context.output_format, &data, output)? {
        context.reporter.report(StorageReadEvent::new(payload))?;
    }

    Ok(())
}

async fn list(context: &Context, path: StoragePath) -> Result<()> {
    let list = context.client.storage().list(path).await?;

    context.reporter.report(StorageListEvent::new(list))?;

    Ok(())
}

async fn remove(context: &Context, path: StoragePath) -> Result<()> {
    context.client.storage().remove(path).await?;

    context.reporter.report(OkEvent::new("storage remove"))?;

    Ok(())
}

async fn mkdir(context: &Context, path: StoragePath) -> Result<()> {
    context.client.storage().mkdir(path).await?;

    context.reporter.report(OkEvent::new("storage mkdir"))?;

    Ok(())
}

async fn rename(context: &Context, path: StoragePath, new_path: StoragePath) -> Result<()> {
    context.client.storage().rename(path, new_path).await?;

    context.reporter.report(OkEvent::new("storage rename"))?;

    Ok(())
}

async fn status(context: &Context) -> Result<()> {
    let status = context.client.storage().status().await?;

    context.reporter.report(StorageStatusEvent::new(status))?;

    Ok(())
}
