use std::fmt;

use busylib::model::settings::{AccessMode, HttpAccessInfo};
use busylib::types::brightness::Brightness;
use busylib::types::device_name::DeviceName;
use busylib::types::volume::Volume;
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::fields::{field, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SettingsAccessEvent(HttpAccessInfo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SettingsNameEvent {
    name: DeviceName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SettingsVolumeEvent {
    volume: Volume,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SettingsBrightnessEvent {
    value: Brightness,
}

impl SettingsAccessEvent {
    pub fn new(access: HttpAccessInfo) -> Self {
        Self(access)
    }
}

impl SettingsNameEvent {
    pub fn new(name: DeviceName) -> Self {
        Self { name }
    }
}

impl SettingsVolumeEvent {
    pub fn new(volume: Volume) -> Self {
        Self { volume }
    }
}

impl SettingsBrightnessEvent {
    pub fn new(value: Brightness) -> Self {
        Self { value }
    }
}

impl From<SettingsAccessEvent> for CliEvent {
    fn from(event: SettingsAccessEvent) -> Self {
        CliEvent::SettingsAccess(event)
    }
}

impl From<SettingsNameEvent> for CliEvent {
    fn from(event: SettingsNameEvent) -> Self {
        CliEvent::SettingsName(event)
    }
}

impl From<SettingsVolumeEvent> for CliEvent {
    fn from(event: SettingsVolumeEvent) -> Self {
        CliEvent::SettingsVolume(event)
    }
}

impl From<SettingsBrightnessEvent> for CliEvent {
    fn from(event: SettingsBrightnessEvent) -> Self {
        CliEvent::SettingsBrightness(event)
    }
}

impl fmt::Display for SettingsAccessEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let access = &self.0;
        let mut fields = vec![field("mode", access_mode_label(access.mode))];

        if let Some(key_valid) = access.key_valid {
            fields.push(field("key valid", key_valid));
        }

        write_fields(f, &fields)
    }
}

impl fmt::Display for SettingsNameEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name.as_str())
    }
}

impl fmt::Display for SettingsVolumeEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.volume.percent())
    }
}

impl fmt::Display for SettingsBrightnessEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn access_mode_label(mode: AccessMode) -> &'static str {
    match mode {
        AccessMode::Disabled => "disabled",
        AccessMode::Enabled => "enabled",
        AccessMode::Key => "key",
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn renders_the_access_configuration() {
        let event = SettingsAccessEvent::new(HttpAccessInfo {
            mode: AccessMode::Key,
            key_valid: Some(true),
        });

        assert_eq!(event.to_string(), "mode: key\nkey valid: true");
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({"event": "settings_access", "mode": "key", "key_valid": true})
        );
    }

    #[test]
    fn renders_single_value_settings_bare() {
        assert_eq!(
            SettingsNameEvent::new(DeviceName::new("BUSY Bar").unwrap()).to_string(),
            "BUSY Bar"
        );
        assert_eq!(
            SettingsVolumeEvent::new(Volume::new(35).unwrap()).to_string(),
            "35%"
        );
        assert_eq!(
            SettingsBrightnessEvent::new(Brightness::Auto).to_string(),
            "auto"
        );
        assert_eq!(
            SettingsBrightnessEvent::new(Brightness::level(40).unwrap()).to_string(),
            "40"
        );
    }
}
