use std::fmt;

use busylib::model::wifi::{
    StatusResponse, WifiIpMethod, WifiIpType, WifiSecurityMethod, WifiState,
};
use serde::Serialize;

use crate::reporter::events::CliEvent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WifiStatusEvent(StatusResponse);

impl WifiStatusEvent {
    pub fn new(status: StatusResponse) -> Self {
        Self(status)
    }
}

impl From<WifiStatusEvent> for CliEvent {
    fn from(event: WifiStatusEvent) -> Self {
        CliEvent::WifiStatus(event)
    }
}

impl fmt::Display for WifiStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = &self.0;

        write!(f, "state: {}", state_label(&status.state))?;

        if let Some(ssid) = &status.ssid {
            write!(f, "\nssid: {ssid}")?;
        }

        if let Some(bssid) = &status.bssid {
            write!(f, "\nbssid: {bssid}")?;
        }

        if let Some(channel) = status.channel {
            write!(f, "\nchannel: {channel}")?;
        }

        if let Some(rssi) = status.rssi {
            write!(f, "\nrssi: {rssi} dBm")?;
        }

        if let Some(security) = &status.security {
            write!(f, "\nsecurity: {}", security_label(security))?;
        }

        if let Some(ip_config) = &status.ip_config {
            if let Some(ip_method) = ip_config.ip_method {
                write!(f, "\nip method: {}", ip_method_label(ip_method))?;
            }

            if let Some(ip_type) = ip_config.ip_type {
                write!(f, "\nip type: {}", ip_type_label(ip_type))?;
            }

            if let Some(address) = &ip_config.address {
                write!(f, "\naddress: {address}")?;
            }
        }

        Ok(())
    }
}

fn state_label(state: &WifiState) -> &str {
    match state {
        WifiState::Unknown => "unknown",
        WifiState::Disconnected => "disconnected",
        WifiState::Connected => "connected",
        WifiState::Connecting => "connecting",
        WifiState::Disconnecting => "disconnecting",
        WifiState::Reconnecting => "reconnecting",
        WifiState::Unrecognized(state) => state,
    }
}

fn security_label(security: &WifiSecurityMethod) -> &str {
    match security {
        WifiSecurityMethod::Open => "Open",
        WifiSecurityMethod::Wpa => "WPA",
        WifiSecurityMethod::Wpa2 => "WPA2",
        WifiSecurityMethod::Wep => "WEP",
        WifiSecurityMethod::WpaWpa2 => "WPA/WPA2",
        WifiSecurityMethod::Wpa3 => "WPA3",
        WifiSecurityMethod::Wpa2Wpa3 => "WPA2/WPA3",
        WifiSecurityMethod::Unsupported => "Unsupported",
        WifiSecurityMethod::Unknown(security) => security,
    }
}

fn ip_method_label(ip_method: WifiIpMethod) -> &'static str {
    match ip_method {
        WifiIpMethod::Dhcp => "dhcp",
        WifiIpMethod::Static => "static",
    }
}

fn ip_type_label(ip_type: WifiIpType) -> &'static str {
    match ip_type {
        WifiIpType::Ipv4 => "ipv4",
        WifiIpType::Ipv6 => "ipv6",
    }
}

#[cfg(test)]
mod tests {
    use busylib::model::wifi::WifiIpConfig;
    use serde_json::json;

    use super::*;

    fn connected() -> WifiStatusEvent {
        WifiStatusEvent::new(StatusResponse {
            state: WifiState::Connected,
            ssid: Some("busy-network".to_string()),
            bssid: Some("a1:b2:c3:d4:e5:f6".to_string()),
            channel: Some(11),
            rssi: Some(-57),
            security: Some(WifiSecurityMethod::Wpa2Wpa3),
            ip_config: Some(WifiIpConfig {
                ip_method: Some(WifiIpMethod::Dhcp),
                ip_type: Some(WifiIpType::Ipv4),
                address: Some("10.0.4.20".to_string()),
            }),
        })
    }

    fn disconnected() -> WifiStatusEvent {
        WifiStatusEvent::new(StatusResponse {
            state: WifiState::Disconnected,
            ssid: None,
            bssid: None,
            channel: None,
            rssi: None,
            security: None,
            ip_config: None,
        })
    }

    fn json_of(event: WifiStatusEvent) -> serde_json::Value {
        serde_json::to_value(CliEvent::from(event)).unwrap()
    }

    #[test]
    fn renders_a_connected_status_as_text() {
        assert_eq!(
            connected().to_string(),
            "state: connected\n\
             ssid: busy-network\n\
             bssid: a1:b2:c3:d4:e5:f6\n\
             channel: 11\n\
             rssi: -57 dBm\n\
             security: WPA2/WPA3\n\
             ip method: dhcp\n\
             ip type: ipv4\n\
             address: 10.0.4.20"
        );
    }

    #[test]
    fn leaves_out_fields_that_are_absent_while_disconnected() {
        assert_eq!(disconnected().to_string(), "state: disconnected");
    }

    #[test]
    fn renders_states_and_security_methods_the_way_the_api_spells_them() {
        let unrecognized = WifiStatusEvent::new(StatusResponse {
            state: WifiState::Unrecognized("rebooting".to_string()),
            security: Some(WifiSecurityMethod::Unknown("WPA4".to_string())),
            ..disconnected().0
        });

        assert_eq!(unrecognized.to_string(), "state: rebooting\nsecurity: WPA4");
        assert_eq!(
            json_of(unrecognized),
            json!({
                "event": "wifi_status",
                "state": "rebooting",
                "ssid": null,
                "bssid": null,
                "channel": null,
                "rssi": null,
                "security": "WPA4",
                "ip_config": null
            })
        );
    }

    #[test]
    fn renders_a_connected_status_as_json() {
        assert_eq!(
            json_of(connected()),
            json!({
                "event": "wifi_status",
                "state": "connected",
                "ssid": "busy-network",
                "bssid": "a1:b2:c3:d4:e5:f6",
                "channel": 11,
                "rssi": -57,
                "security": "WPA2/WPA3",
                "ip_config": {
                    "ip_method": "dhcp",
                    "ip_type": "ipv4",
                    "address": "10.0.4.20"
                }
            })
        );
    }

    #[test]
    fn renders_a_disconnected_status_as_json() {
        assert_eq!(
            json_of(disconnected()),
            json!({
                "event": "wifi_status",
                "state": "disconnected",
                "ssid": null,
                "bssid": null,
                "channel": null,
                "rssi": null,
                "security": null,
                "ip_config": null
            })
        );
    }
}
