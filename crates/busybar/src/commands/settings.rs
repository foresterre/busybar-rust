use busylib::model::settings::HttpAccess;
use busylib::types::access_key::AccessKey;
use busylib::types::brightness::Brightness;
use busylib::types::device_name::DeviceName;
use busylib::types::volume::Volume;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::{CliError, Result};
use crate::reporter::{
    OkEvent, SettingsAccessEvent, SettingsBrightnessEvent, SettingsNameEvent, SettingsVolumeEvent,
};
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
        #[arg(long, short = 'k', value_name = "KEY", required_if_eq("mode", "key"))]
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
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            SettingsCommand::Access => access(context).await,
            SettingsCommand::SetAccess { mode, key } => set_access(context, mode, key).await,
            SettingsCommand::Name => name(context).await,
            SettingsCommand::SetName { name } => set_name(context, name).await,
            SettingsCommand::Volume => volume(context).await,
            SettingsCommand::SetVolume { volume, silent } => {
                set_volume(context, volume, silent).await
            }
            SettingsCommand::Brightness => brightness(context).await,
            SettingsCommand::SetBrightness { value } => set_brightness(context, value).await,
        }
    }
}

async fn access(context: &Context) -> Result<()> {
    let access = context.client.settings().access().await?;

    context.reporter.report(SettingsAccessEvent::new(access))?;

    Ok(())
}

async fn set_access(context: &Context, mode: AccessModeArg, key: Option<AccessKey>) -> Result<()> {
    let access = match (mode, key) {
        (AccessModeArg::Disabled, _) => HttpAccess::Disabled,
        (AccessModeArg::Enabled, _) => HttpAccess::Enabled,
        (AccessModeArg::Key, Some(key)) => HttpAccess::Key(key),
        (AccessModeArg::Key, None) => {
            return Err(CliError::Usage("--key is required for the key mode"));
        }
    };

    context.client.settings().set_access(&access).await?;

    context
        .reporter
        .report(OkEvent::new("settings set-access"))?;

    Ok(())
}

async fn name(context: &Context) -> Result<()> {
    let name = context.client.settings().name().await?;

    context.reporter.report(SettingsNameEvent::new(name))?;

    Ok(())
}

async fn set_name(context: &Context, name: DeviceName) -> Result<()> {
    context.client.settings().set_name(name).await?;

    context.reporter.report(OkEvent::new("settings set-name"))?;

    Ok(())
}

async fn volume(context: &Context) -> Result<()> {
    let volume = context.client.settings().volume().await?;

    context.reporter.report(SettingsVolumeEvent::new(volume))?;

    Ok(())
}

async fn set_volume(context: &Context, volume: Volume, silent: bool) -> Result<()> {
    context.client.settings().set_volume(volume, silent).await?;

    context
        .reporter
        .report(OkEvent::new("settings set-volume"))?;

    Ok(())
}

async fn brightness(context: &Context) -> Result<()> {
    let brightness = context.client.settings().brightness().await?;

    context
        .reporter
        .report(SettingsBrightnessEvent::new(brightness))?;

    Ok(())
}

async fn set_brightness(context: &Context, value: Brightness) -> Result<()> {
    context.client.settings().set_brightness(value).await?;

    context
        .reporter
        .report(OkEvent::new("settings set-brightness"))?;

    Ok(())
}
