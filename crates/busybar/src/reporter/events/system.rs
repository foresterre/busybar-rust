use std::fmt;

use busylib::model::system::{
    FirmwareSecurity, PowerState, Status, StatusDevice, StatusFirmware, StatusPower, StatusSystem,
    TransportType,
};
use serde::Serialize;

use crate::reporter::events::CliEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SystemVersionEvent {
    api_semver: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SystemTransportEvent {
    transport: TransportType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SystemStatusEvent(Status);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SystemStatusDeviceEvent(StatusDevice);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SystemStatusFirmwareEvent(StatusFirmware);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SystemStatusSystemEvent(StatusSystem);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SystemStatusPowerEvent(StatusPower);

impl SystemVersionEvent {
    pub fn new(api_semver: String) -> Self {
        Self { api_semver }
    }
}

impl SystemTransportEvent {
    pub fn new(transport: TransportType) -> Self {
        Self { transport }
    }
}

impl SystemStatusEvent {
    pub fn new(status: Status) -> Self {
        Self(status)
    }
}

impl SystemStatusDeviceEvent {
    pub fn new(device: StatusDevice) -> Self {
        Self(device)
    }
}

impl SystemStatusFirmwareEvent {
    pub fn new(firmware: StatusFirmware) -> Self {
        Self(firmware)
    }
}

impl SystemStatusSystemEvent {
    pub fn new(system: StatusSystem) -> Self {
        Self(system)
    }
}

impl SystemStatusPowerEvent {
    pub fn new(power: StatusPower) -> Self {
        Self(power)
    }
}

impl From<SystemVersionEvent> for CliEvent {
    fn from(event: SystemVersionEvent) -> Self {
        CliEvent::SystemVersion(event)
    }
}

impl From<SystemTransportEvent> for CliEvent {
    fn from(event: SystemTransportEvent) -> Self {
        CliEvent::SystemTransport(event)
    }
}

impl From<SystemStatusEvent> for CliEvent {
    fn from(event: SystemStatusEvent) -> Self {
        CliEvent::SystemStatus(Box::new(event))
    }
}

impl From<SystemStatusDeviceEvent> for CliEvent {
    fn from(event: SystemStatusDeviceEvent) -> Self {
        CliEvent::SystemStatusDevice(event)
    }
}

impl From<SystemStatusFirmwareEvent> for CliEvent {
    fn from(event: SystemStatusFirmwareEvent) -> Self {
        CliEvent::SystemStatusFirmware(event)
    }
}

impl From<SystemStatusSystemEvent> for CliEvent {
    fn from(event: SystemStatusSystemEvent) -> Self {
        CliEvent::SystemStatusSystem(event)
    }
}

impl From<SystemStatusPowerEvent> for CliEvent {
    fn from(event: SystemStatusPowerEvent) -> Self {
        CliEvent::SystemStatusPower(event)
    }
}

impl fmt::Display for SystemVersionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.api_semver)
    }
}

impl fmt::Display for SystemTransportEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(transport_label(&self.transport))
    }
}

impl fmt::Display for SystemStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = &self.0;
        let mut fields = Vec::new();

        if let Some(device) = &status.device {
            fields.extend(prefixed("device", device_fields(device)));
        }

        if let Some(firmware) = &status.firmware {
            fields.extend(prefixed("firmware", firmware_fields(firmware)));
        }

        if let Some(system) = &status.system {
            fields.extend(prefixed("system", system_fields(system)));
        }

        if let Some(power) = &status.power {
            fields.extend(prefixed("power", power_fields(power)));
        }

        write_fields(f, &fields)
    }
}

impl fmt::Display for SystemStatusDeviceEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_fields(f, &device_fields(&self.0))
    }
}

impl fmt::Display for SystemStatusFirmwareEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_fields(f, &firmware_fields(&self.0))
    }
}

impl fmt::Display for SystemStatusSystemEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_fields(f, &system_fields(&self.0))
    }
}

impl fmt::Display for SystemStatusPowerEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_fields(f, &power_fields(&self.0))
    }
}

type Field = (String, String);

fn field(key: &str, value: impl fmt::Display) -> Field {
    (key.to_owned(), value.to_string())
}

fn prefixed(prefix: &str, fields: Vec<Field>) -> Vec<Field> {
    fields
        .into_iter()
        .map(|(key, value)| (format!("{prefix} {key}"), value))
        .collect()
}

fn write_fields(f: &mut fmt::Formatter<'_>, fields: &[Field]) -> fmt::Result {
    for (index, (key, value)) in fields.iter().enumerate() {
        if index > 0 {
            f.write_str("\n")?;
        }

        write!(f, "{key}: {value}")?;
    }

    Ok(())
}

fn device_fields(device: &StatusDevice) -> Vec<Field> {
    let mut fields = vec![
        field("serial number", &device.serial_number),
        field("usb mac", &device.usb_mac),
    ];

    if let Some(wifi_mac) = &device.wifi_mac {
        fields.push(field("wifi mac", wifi_mac));
    }

    if let Some(ble_mac) = &device.ble_mac {
        fields.push(field("ble mac", ble_mac));
    }

    fields.push(field("otp valid", device.otp_valid));

    if let Some(otp_model) = &device.otp_model {
        fields.push(field("otp model", otp_model));
    }

    if let Some(otp_timestamp) = device.otp_timestamp {
        fields.push(field("otp timestamp", otp_timestamp));
    }

    fields.push(field(
        "firmware security",
        firmware_security_label(&device.firmware_security),
    ));

    fields
}

fn firmware_fields(firmware: &StatusFirmware) -> Vec<Field> {
    let mut fields = vec![
        field("version", &firmware.version),
        field("target", firmware.target),
        field("branch", &firmware.branch),
        field("build date", &firmware.build_date),
        field("commit hash", &firmware.commit_hash),
        field("intercom version", &firmware.intercom_version),
    ];

    if let Some(nwp_version) = &firmware.nwp_version {
        fields.push(field("nwp version", nwp_version));
    }

    if let Some(matter_version) = &firmware.matter_version {
        fields.push(field("matter version", matter_version));
    }

    fields
}

fn system_fields(system: &StatusSystem) -> Vec<Field> {
    vec![
        field("api semver", &system.api_semver),
        field("uptime", &system.uptime),
        field("boot time", system.boot_time),
        field("auto update enabled", system.auto_update_enabled),
    ]
}

fn power_fields(power: &StatusPower) -> Vec<Field> {
    vec![
        field("state", power_state_label(&power.state)),
        field("battery charge", format_args!("{}%", power.battery_charge)),
        field(
            "battery voltage",
            format_args!("{} mV", power.battery_voltage),
        ),
        field(
            "battery current",
            format_args!("{} mA", power.battery_current),
        ),
        field("usb voltage", format_args!("{} mV", power.usb_voltage)),
    ]
}

fn transport_label(transport: &TransportType) -> &str {
    match transport {
        TransportType::Usb => "usb",
        TransportType::Wifi => "wifi",
        TransportType::Unknown(transport) => transport,
    }
}

fn firmware_security_label(security: &FirmwareSecurity) -> &str {
    match security {
        FirmwareSecurity::Secure => "secure",
        FirmwareSecurity::Insecure => "insecure",
        FirmwareSecurity::Other => "other",
        FirmwareSecurity::Unknown => "unknown",
        FirmwareSecurity::Unrecognized(security) => security,
    }
}

fn power_state_label(state: &PowerState) -> &str {
    match state {
        PowerState::Discharging => "discharging",
        PowerState::Charging => "charging",
        PowerState::Charged => "charged",
        PowerState::Unknown(state) => state,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn device() -> StatusDevice {
        StatusDevice {
            serial_number: "2034305532325009002d001d".to_string(),
            usb_mac: "0c:fa:22:00:07:84".to_string(),
            wifi_mac: Some("0c:fa:22:00:07:85".to_string()),
            ble_mac: Some("0c:fa:22:00:07:86".to_string()),
            otp_valid: true,
            otp_model: Some("BB.1".to_string()),
            otp_timestamp: Some(1782547210),
            firmware_security: FirmwareSecurity::Secure,
        }
    }

    fn firmware() -> StatusFirmware {
        StatusFirmware {
            version: "1.4.0".to_string(),
            target: 7,
            branch: "main".to_string(),
            build_date: "2026-06-01".to_string(),
            commit_hash: "abc1234-dirty".to_string(),
            intercom_version: "intercom".to_string(),
            nwp_version: None,
            matter_version: None,
        }
    }

    fn system() -> StatusSystem {
        StatusSystem {
            api_semver: "25.0.0".to_string(),
            uptime: "3h 12m".to_string(),
            boot_time: 1782547210,
            auto_update_enabled: false,
        }
    }

    fn power() -> StatusPower {
        StatusPower {
            state: PowerState::Charging,
            battery_charge: 87,
            battery_voltage: 4012,
            battery_current: -230,
            usb_voltage: 5100,
        }
    }

    #[test]
    fn renders_single_value_events_bare_so_they_can_be_piped() {
        assert_eq!(
            SystemVersionEvent::new("25.0.0".to_string()).to_string(),
            "25.0.0"
        );
        assert_eq!(
            SystemTransportEvent::new(TransportType::Usb).to_string(),
            "usb"
        );
        assert_eq!(
            SystemTransportEvent::new(TransportType::Unknown("ethernet".to_string())).to_string(),
            "ethernet"
        );
    }

    #[test]
    fn renders_single_value_events_as_a_named_json_field() {
        assert_eq!(
            serde_json::to_value(CliEvent::from(SystemVersionEvent::new(
                "25.0.0".to_string()
            )))
            .unwrap(),
            json!({"event": "system_version", "api_semver": "25.0.0"})
        );
        assert_eq!(
            serde_json::to_value(CliEvent::from(SystemTransportEvent::new(
                TransportType::Wifi
            )))
            .unwrap(),
            json!({"event": "system_transport", "transport": "wifi"})
        );
    }

    #[test]
    fn renders_device_information_as_text() {
        assert_eq!(
            SystemStatusDeviceEvent::new(device()).to_string(),
            "serial number: 2034305532325009002d001d\n\
             usb mac: 0c:fa:22:00:07:84\n\
             wifi mac: 0c:fa:22:00:07:85\n\
             ble mac: 0c:fa:22:00:07:86\n\
             otp valid: true\n\
             otp model: BB.1\n\
             otp timestamp: 1782547210\n\
             firmware security: secure"
        );
    }

    #[test]
    fn leaves_out_device_and_firmware_fields_the_device_does_not_report() {
        let bare = SystemStatusDeviceEvent::new(StatusDevice {
            wifi_mac: None,
            ble_mac: None,
            otp_model: None,
            otp_timestamp: None,
            firmware_security: FirmwareSecurity::Unrecognized("partial".to_string()),
            ..device()
        });

        assert_eq!(
            bare.to_string(),
            "serial number: 2034305532325009002d001d\n\
             usb mac: 0c:fa:22:00:07:84\n\
             otp valid: true\n\
             firmware security: partial"
        );

        assert_eq!(
            SystemStatusFirmwareEvent::new(firmware()).to_string(),
            "version: 1.4.0\n\
             target: 7\n\
             branch: main\n\
             build date: 2026-06-01\n\
             commit hash: abc1234-dirty\n\
             intercom version: intercom"
        );
    }

    #[test]
    fn renders_units_for_power_readings() {
        assert_eq!(
            SystemStatusPowerEvent::new(power()).to_string(),
            "state: charging\n\
             battery charge: 87%\n\
             battery voltage: 4012 mV\n\
             battery current: -230 mA\n\
             usb voltage: 5100 mV"
        );
    }

    #[test]
    fn renders_system_information_as_text() {
        assert_eq!(
            SystemStatusSystemEvent::new(system()).to_string(),
            "api semver: 25.0.0\n\
             uptime: 3h 12m\n\
             boot time: 1782547210\n\
             auto update enabled: false"
        );
    }

    #[test]
    fn prefixes_each_section_of_the_combined_status() {
        let status = SystemStatusEvent::new(Status {
            device: None,
            firmware: Some(firmware()),
            system: Some(system()),
            power: Some(power()),
        });

        assert_eq!(
            status.to_string(),
            "firmware version: 1.4.0\n\
             firmware target: 7\n\
             firmware branch: main\n\
             firmware build date: 2026-06-01\n\
             firmware commit hash: abc1234-dirty\n\
             firmware intercom version: intercom\n\
             system api semver: 25.0.0\n\
             system uptime: 3h 12m\n\
             system boot time: 1782547210\n\
             system auto update enabled: false\n\
             power state: charging\n\
             power battery charge: 87%\n\
             power battery voltage: 4012 mV\n\
             power battery current: -230 mA\n\
             power usb voltage: 5100 mV"
        );
    }

    #[test]
    fn keeps_the_response_shape_in_json() {
        let status = SystemStatusEvent::new(Status {
            device: Some(device()),
            firmware: None,
            system: None,
            power: Some(power()),
        });

        assert_eq!(
            serde_json::to_value(CliEvent::from(status)).unwrap(),
            json!({
                "event": "system_status",
                "device": {
                    "serial_number": "2034305532325009002d001d",
                    "usb_mac": "0c:fa:22:00:07:84",
                    "wifi_mac": "0c:fa:22:00:07:85",
                    "ble_mac": "0c:fa:22:00:07:86",
                    "otp_valid": true,
                    "otp_model": "BB.1",
                    "otp_timestamp": 1782547210,
                    "firmware_security": "secure"
                },
                "firmware": null,
                "system": null,
                "power": {
                    "state": "charging",
                    "battery_charge": 87,
                    "battery_voltage": 4012,
                    "battery_current": -230,
                    "usb_voltage": 5100
                }
            })
        );
    }
}
