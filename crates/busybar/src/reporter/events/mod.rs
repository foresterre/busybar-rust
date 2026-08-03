mod account;
mod binary;
mod ble;
mod busy;
mod fields;
mod ok;
mod settings;
mod smart_home;
mod storage;
mod streaming;
mod system;
mod time;
mod unsupported;
mod updater;
mod wifi;

use std::fmt;

use serde::Serialize;

pub use crate::reporter::events::account::{
    AccountBackendEvent, AccountInfoEvent, AccountStatusEvent,
};
pub use crate::reporter::events::binary::Payload;
pub use crate::reporter::events::ble::BleStatusEvent;
pub use crate::reporter::events::busy::{BusyProfileEvent, BusySnapshotEvent};
pub use crate::reporter::events::ok::OkEvent;
pub use crate::reporter::events::settings::{
    SettingsAccessEvent, SettingsBrightnessEvent, SettingsNameEvent, SettingsVolumeEvent,
};
pub use crate::reporter::events::smart_home::{
    SmartHomePairingEvent, SmartHomeStartPairingEvent, SmartHomeSwitchEvent,
};
pub use crate::reporter::events::storage::{
    StorageListEvent, StorageReadEvent, StorageStatusEvent,
};
pub use crate::reporter::events::streaming::{
    FramePayload, StreamingScreenEvent, StreamingStatusEvent,
};
pub use crate::reporter::events::system::{
    SystemLogDumpEvent, SystemStatusDeviceEvent, SystemStatusEvent, SystemStatusFirmwareEvent,
    SystemStatusPowerEvent, SystemStatusSystemEvent, SystemTransportEvent, SystemVersionEvent,
};
pub use crate::reporter::events::time::{TimeNowEvent, TimeTimezoneEvent, TimeTzlistEvent};
pub use crate::reporter::events::unsupported::UnsupportedEvent;
pub use crate::reporter::events::updater::{
    UpdaterAutoupdateEvent, UpdaterChangelogEvent, UpdaterStatusEvent,
};
pub use crate::reporter::events::wifi::WifiStatusEvent;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum CliEvent {
    Ok(OkEvent),
    Unsupported(UnsupportedEvent),

    AccountInfo(AccountInfoEvent),
    AccountStatus(AccountStatusEvent),
    AccountBackend(AccountBackendEvent),

    BleStatus(BleStatusEvent),

    BusySnapshot(Box<BusySnapshotEvent>),
    BusyProfile(Box<BusyProfileEvent>),

    SettingsAccess(SettingsAccessEvent),
    SettingsName(SettingsNameEvent),
    SettingsVolume(SettingsVolumeEvent),
    SettingsBrightness(SettingsBrightnessEvent),

    SmartHomePairing(SmartHomePairingEvent),
    SmartHomeStartPairing(SmartHomeStartPairingEvent),
    SmartHomeSwitch(SmartHomeSwitchEvent),

    StorageList(StorageListEvent),
    StorageStatus(StorageStatusEvent),
    StorageRead(StorageReadEvent),

    StreamingScreen(StreamingScreenEvent),
    StreamingStatus(Box<StreamingStatusEvent>),

    SystemVersion(SystemVersionEvent),
    SystemTransport(SystemTransportEvent),
    SystemStatus(Box<SystemStatusEvent>),
    SystemStatusDevice(SystemStatusDeviceEvent),
    SystemStatusFirmware(SystemStatusFirmwareEvent),
    SystemStatusSystem(SystemStatusSystemEvent),
    SystemStatusPower(SystemStatusPowerEvent),
    SystemLogDump(SystemLogDumpEvent),

    TimeNow(TimeNowEvent),
    TimeTimezone(TimeTimezoneEvent),
    TimeTzlist(TimeTzlistEvent),

    UpdaterStatus(Box<UpdaterStatusEvent>),
    UpdaterChangelog(UpdaterChangelogEvent),
    UpdaterAutoupdate(UpdaterAutoupdateEvent),

    WifiStatus(WifiStatusEvent),
}

impl fmt::Display for CliEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliEvent::Ok(event) => event.fmt(f),
            CliEvent::Unsupported(event) => event.fmt(f),
            CliEvent::AccountInfo(event) => event.fmt(f),
            CliEvent::AccountStatus(event) => event.fmt(f),
            CliEvent::AccountBackend(event) => event.fmt(f),
            CliEvent::BleStatus(event) => event.fmt(f),
            CliEvent::BusySnapshot(event) => event.fmt(f),
            CliEvent::BusyProfile(event) => event.fmt(f),
            CliEvent::SettingsAccess(event) => event.fmt(f),
            CliEvent::SettingsName(event) => event.fmt(f),
            CliEvent::SettingsVolume(event) => event.fmt(f),
            CliEvent::SettingsBrightness(event) => event.fmt(f),
            CliEvent::SmartHomePairing(event) => event.fmt(f),
            CliEvent::SmartHomeStartPairing(event) => event.fmt(f),
            CliEvent::SmartHomeSwitch(event) => event.fmt(f),
            CliEvent::StorageList(event) => event.fmt(f),
            CliEvent::StorageStatus(event) => event.fmt(f),
            CliEvent::StorageRead(event) => event.fmt(f),
            CliEvent::StreamingScreen(event) => event.fmt(f),
            CliEvent::StreamingStatus(event) => event.fmt(f),
            CliEvent::SystemVersion(event) => event.fmt(f),
            CliEvent::SystemTransport(event) => event.fmt(f),
            CliEvent::SystemStatus(event) => event.fmt(f),
            CliEvent::SystemStatusDevice(event) => event.fmt(f),
            CliEvent::SystemStatusFirmware(event) => event.fmt(f),
            CliEvent::SystemStatusSystem(event) => event.fmt(f),
            CliEvent::SystemStatusPower(event) => event.fmt(f),
            CliEvent::SystemLogDump(event) => event.fmt(f),
            CliEvent::TimeNow(event) => event.fmt(f),
            CliEvent::TimeTimezone(event) => event.fmt(f),
            CliEvent::TimeTzlist(event) => event.fmt(f),
            CliEvent::UpdaterStatus(event) => event.fmt(f),
            CliEvent::UpdaterChangelog(event) => event.fmt(f),
            CliEvent::UpdaterAutoupdate(event) => event.fmt(f),
            CliEvent::WifiStatus(event) => event.fmt(f),
        }
    }
}
