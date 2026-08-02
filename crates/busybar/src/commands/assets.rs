use std::path::PathBuf;

use busylib::types::app_name::AppName;
use busylib::types::asset_name::AssetName;
use clap::Subcommand;

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
}

impl AssetsCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
