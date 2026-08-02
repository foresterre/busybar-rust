use std::fmt;

use busylib::ApiPrefix;
use busylib::model::assets::Screen;
use busylib::model::busy::BusyProfileSlot;
use busylib::model::input::Key;
use busylib::model::settings::AccessMode;
use busylib::model::smart_home::SwitchStartup;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ApiPrefixArg {
    Device,
    Cloud,
}

impl From<ApiPrefixArg> for ApiPrefix {
    fn from(prefix: ApiPrefixArg) -> Self {
        match prefix {
            ApiPrefixArg::Device => ApiPrefix::Device,
            ApiPrefixArg::Cloud => ApiPrefix::Cloud,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum KeyArg {
    Up,
    Down,
    Ok,
    Back,
    Start,
    Busy,
    Custom,
    Off,
    Apps,
    Settings,
}

impl From<KeyArg> for Key {
    fn from(key: KeyArg) -> Self {
        match key {
            KeyArg::Up => Key::Up,
            KeyArg::Down => Key::Down,
            KeyArg::Ok => Key::Ok,
            KeyArg::Back => Key::Back,
            KeyArg::Start => Key::Start,
            KeyArg::Busy => Key::Busy,
            KeyArg::Custom => Key::Custom,
            KeyArg::Off => Key::Off,
            KeyArg::Apps => Key::Apps,
            KeyArg::Settings => Key::Settings,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ScreenArg {
    Front,
    Back,
}

impl fmt::Display for ScreenArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScreenArg::Front => f.write_str("front"),
            ScreenArg::Back => f.write_str("back"),
        }
    }
}

impl From<ScreenArg> for Screen {
    fn from(screen: ScreenArg) -> Self {
        match screen {
            ScreenArg::Front => Screen::Front,
            ScreenArg::Back => Screen::Back,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum SlotArg {
    Busy,
    Custom,
}

impl From<SlotArg> for BusyProfileSlot {
    fn from(slot: SlotArg) -> Self {
        match slot {
            SlotArg::Busy => BusyProfileSlot::Busy,
            SlotArg::Custom => BusyProfileSlot::Custom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum AccessModeArg {
    Disabled,
    Enabled,
    Key,
}

impl From<AccessModeArg> for AccessMode {
    fn from(mode: AccessModeArg) -> Self {
        match mode {
            AccessModeArg::Disabled => AccessMode::Disabled,
            AccessModeArg::Enabled => AccessMode::Enabled,
            AccessModeArg::Key => AccessMode::Key,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum SwitchStateArg {
    On,
    Off,
}

impl From<SwitchStateArg> for bool {
    fn from(state: SwitchStateArg) -> Self {
        matches!(state, SwitchStateArg::On)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum SwitchStartupArg {
    Off,
    On,
    Toggle,
    Last,
}

impl From<SwitchStartupArg> for SwitchStartup {
    fn from(startup: SwitchStartupArg) -> Self {
        match startup {
            SwitchStartupArg::Off => SwitchStartup::Off,
            SwitchStartupArg::On => SwitchStartup::On,
            SwitchStartupArg::Toggle => SwitchStartup::Toggle,
            SwitchStartupArg::Last => SwitchStartup::Last,
        }
    }
}
