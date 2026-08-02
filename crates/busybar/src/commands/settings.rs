use busylib::types::access_key::AccessKey;
use busylib::types::brightness::Brightness;
use busylib::types::device_name::DeviceName;
use busylib::types::volume::Volume;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::values::AccessModeArg;

#[derive(Debug, Subcommand)]
pub enum SettingsCommand {
    /// Show the HTTP API access configuration
    Access,

    /// Set how the HTTP API may be reached over Wi-Fi
    SetAccess {
        /// Access mode
        #[arg(value_enum, value_name = "MODE")]
        mode: AccessModeArg,

        /// Access key of 4 to 10 digits, required for the key mode
        #[arg(long, short = 'k', value_name = "KEY")]
        key: Option<AccessKey>,
    },

    /// Show the device name
    Name,

    /// Set the device name
    SetName {
        /// New device name
        #[arg(value_name = "NAME")]
        name: DeviceName,
    },

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

    /// Show the display brightness
    Brightness,

    /// Set the display brightness
    SetBrightness {
        /// Percentage between 0 and 100, or auto
        #[arg(value_name = "VALUE")]
        value: Brightness,
    },
}

impl SettingsCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
