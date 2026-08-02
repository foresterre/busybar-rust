use std::path::PathBuf;

use busylib::types::app_name::AppName;
use busylib::types::asset_name::AssetName;
use busylib::types::asset_path::AssetPath;
use busylib::types::stock_path::StockPath;
use clap::{ArgGroup, Subcommand};

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum AssetsCommand {
    /// Upload a file into an application's assets
    Upload {
        /// Application name
        #[arg(long, short = 'a', value_name = "NAME")]
        app: AppName,

        /// Asset file name on the device
        #[arg(long, short = 'f', value_name = "NAME")]
        file: Option<AssetName>,

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
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
