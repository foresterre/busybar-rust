use std::fmt;

use busylib::model::account::{AccountBackend, AccountInfo, ClientCertType, MqttStatus};
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::fields::{Field, field, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountInfoEvent(AccountInfo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountStatusEvent {
    status: MqttStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountBackendEvent(AccountBackend);

impl AccountInfoEvent {
    pub fn new(info: AccountInfo) -> Self {
        Self(info)
    }
}

impl AccountStatusEvent {
    pub fn new(status: MqttStatus) -> Self {
        Self { status }
    }
}

impl AccountBackendEvent {
    pub fn new(backend: AccountBackend) -> Self {
        Self(backend)
    }
}

impl From<AccountInfoEvent> for CliEvent {
    fn from(event: AccountInfoEvent) -> Self {
        CliEvent::AccountInfo(event)
    }
}

impl From<AccountStatusEvent> for CliEvent {
    fn from(event: AccountStatusEvent) -> Self {
        CliEvent::AccountStatus(event)
    }
}

impl From<AccountBackendEvent> for CliEvent {
    fn from(event: AccountBackendEvent) -> Self {
        CliEvent::AccountBackend(event)
    }
}

impl fmt::Display for AccountInfoEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info = &self.0;
        let mut fields = Vec::new();

        if let Some(linked) = info.linked {
            fields.push(field("linked", linked));
        }

        if let Some(id) = &info.id {
            fields.push(field("id", id));
        }

        if let Some(email) = &info.email {
            fields.push(field("email", email));
        }

        if let Some(user_id) = &info.user_id {
            fields.push(field("user id", user_id));
        }

        write_fields(f, &fields)
    }
}

impl fmt::Display for AccountStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(mqtt_status_label(&self.status))
    }
}

impl fmt::Display for AccountBackendEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let backend = &self.0;
        let fields: Vec<Field> = vec![
            field("server url", &backend.server_url),
            field(
                "client cert type",
                client_cert_type_label(&backend.client_cert_type),
            ),
            field("ignore server cert", backend.ignore_server_cert),
        ];

        write_fields(f, &fields)
    }
}

fn mqtt_status_label(status: &MqttStatus) -> &str {
    match status {
        MqttStatus::Error => "error",
        MqttStatus::Disconnected => "disconnected",
        MqttStatus::Connected => "connected",
        MqttStatus::Unknown(status) => status,
    }
}

fn client_cert_type_label(cert: &ClientCertType) -> &str {
    match cert {
        ClientCertType::Default => "default",
        ClientCertType::Custom => "custom",
        ClientCertType::None => "none",
        ClientCertType::Unknown(cert) => cert,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn leaves_out_account_fields_the_device_does_not_report() {
        let event = AccountInfoEvent::new(AccountInfo {
            linked: Some(true),
            id: Some("ee7857a2".to_string()),
            email: None,
            user_id: None,
        });

        assert_eq!(event.to_string(), "linked: true\nid: ee7857a2");
    }

    #[test]
    fn renders_the_cloud_connection_state_bare() {
        assert_eq!(
            AccountStatusEvent::new(MqttStatus::Connected).to_string(),
            "connected"
        );
        assert_eq!(
            AccountStatusEvent::new(MqttStatus::Unknown("reconnecting".to_string())).to_string(),
            "reconnecting"
        );
    }

    #[test]
    fn renders_the_backend_configuration() {
        let event = AccountBackendEvent::new(AccountBackend {
            server_url: "default".to_string(),
            client_cert_type: ClientCertType::Default,
            ignore_server_cert: false,
        });

        assert_eq!(
            event.to_string(),
            "server url: default\nclient cert type: default\nignore server cert: false"
        );
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({
                "event": "account_backend",
                "server_url": "default",
                "client_cert_type": "default",
                "ignore_server_cert": false
            })
        );
    }
}
