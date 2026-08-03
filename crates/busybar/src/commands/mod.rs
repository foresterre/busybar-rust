mod account;
mod assets;
mod ble;
mod busy;
mod input;
mod mirror;
mod settings;
mod smart_home;
mod storage;
mod streaming;
mod system;
mod time;
mod updater;
mod wifi;

use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::types::screen_arg::ScreenArg;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Linked account and cloud connection
    Account {
        #[command(subcommand)]
        command: account::AccountCommand,
    },

    /// Application assets, audio playback and drawing
    Assets {
        #[command(subcommand)]
        command: assets::AssetsCommand,
    },

    /// Bluetooth Low Energy
    Ble {
        #[command(subcommand)]
        command: ble::BleCommand,
    },

    /// BUSY timer snapshots and profiles
    Busy {
        #[command(subcommand)]
        command: busy::BusyCommand,
    },

    /// Synthetic key presses
    Input {
        #[command(subcommand)]
        command: input::InputCommand,
    },

    /// Mirror a screen of the device in the terminal, until interrupted with ctrl-c
    Mirror {
        /// Screen to mirror
        #[arg(long, value_enum, default_value_t = ScreenArg::Front)]
        screen: ScreenArg,

        /// Do not space the pixels of front frames out on the black raster of the matrix
        #[arg(long)]
        no_screen_raster: bool,
    },

    /// Device name, HTTP API access, volume and brightness
    Settings {
        #[command(subcommand)]
        command: settings::SettingsCommand,
    },

    /// Matter pairing and the emulated switch
    #[command(name = "smart-home")]
    SmartHome {
        #[command(subcommand)]
        command: smart_home::SmartHomeCommand,
    },

    /// Files on the internal storage
    Storage {
        #[command(subcommand)]
        command: storage::StorageCommand,
    },

    /// Screen capture
    Streaming {
        #[command(subcommand)]
        command: streaming::StreamingCommand,
    },

    /// Device, firmware, system and power information
    System {
        #[command(subcommand)]
        command: system::SystemCommand,
    },

    /// Clock and time zone
    Time {
        #[command(subcommand)]
        command: time::TimeCommand,
    },

    /// Firmware updates
    Updater {
        #[command(subcommand)]
        command: updater::UpdaterCommand,
    },

    /// Wi-Fi connection
    Wifi {
        #[command(subcommand)]
        command: wifi::WifiCommand,
    },
}

impl Command {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            Command::Account { command } => command.run(context).await,
            Command::Assets { command } => command.run(context).await,
            Command::Ble { command } => command.run(context).await,
            Command::Busy { command } => command.run(context).await,
            Command::Input { command } => command.run(context).await,
            Command::Mirror {
                screen,
                no_screen_raster,
            } => mirror::run(context, screen, no_screen_raster).await,
            Command::Settings { command } => command.run(context).await,
            Command::SmartHome { command } => command.run(context).await,
            Command::Storage { command } => command.run(context).await,
            Command::Streaming { command } => command.run(context).await,
            Command::System { command } => command.run(context).await,
            Command::Time { command } => command.run(context).await,
            Command::Updater { command } => command.run(context).await,
            Command::Wifi { command } => command.run(context).await,
        }
    }
}
