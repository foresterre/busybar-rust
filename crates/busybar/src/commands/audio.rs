use busylib::{AppName, AssetPath, StockPath, Volume};
use clap::{ArgGroup, Subcommand};

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum AudioCommand {
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

    /// Show the audio volume
    Volume,

    /// Set the audio volume
    SetVolume {
        /// Volume between 0 and 100
        #[arg(value_name = "PERCENT")]
        volume: Volume,

        /// Do not play the volume change sound
        #[arg(long)]
        silent: bool,
    },
}

impl AudioCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
