mod account;
mod assets;
mod audio;
mod ble;
mod busy;
mod display;
mod input;
mod settings;
mod smart_home;
mod storage;
mod system;
mod time;
mod updater;
mod wifi;

use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Linked account and cloud connection
    Account {
        #[command(subcommand)]
        command: account::AccountCommand,
    },

    /// Application assets on the device
    Assets {
        #[command(subcommand)]
        command: assets::AssetsCommand,
    },

    /// Audio playback and volume
    Audio {
        #[command(subcommand)]
        command: audio::AudioCommand,
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

    /// Drawing, brightness and screen capture
    Display {
        #[command(subcommand)]
        command: display::DisplayCommand,
    },

    /// Synthetic key presses
    Input {
        #[command(subcommand)]
        command: input::InputCommand,
    },

    /// Device name and HTTP API access
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
            Command::Audio { command } => command.run(context).await,
            Command::Ble { command } => command.run(context).await,
            Command::Busy { command } => command.run(context).await,
            Command::Display { command } => command.run(context).await,
            Command::Input { command } => command.run(context).await,
            Command::Settings { command } => command.run(context).await,
            Command::SmartHome { command } => command.run(context).await,
            Command::Storage { command } => command.run(context).await,
            Command::System { command } => command.run(context).await,
            Command::Time { command } => command.run(context).await,
            Command::Updater { command } => command.run(context).await,
            Command::Wifi { command } => command.run(context).await,
        }
    }
}
