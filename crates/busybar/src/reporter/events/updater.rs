use std::fmt;

use busylib::model::updater::{
    AutoupdateSettings, CheckEvent, CheckResult, CheckStatus, DownloadProgress, InstallResult,
    InstallStatus, UpdateAction, UpdateEvent, UpdateStatus,
};
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::fields::{Field, field, prefixed, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdaterStatusEvent(UpdateStatus);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdaterChangelogEvent {
    changelog: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UpdaterAutoupdateEvent(AutoupdateSettings);

impl UpdaterStatusEvent {
    pub fn new(status: UpdateStatus) -> Self {
        Self(status)
    }
}

impl UpdaterChangelogEvent {
    pub fn new(changelog: String) -> Self {
        Self { changelog }
    }
}

impl UpdaterAutoupdateEvent {
    pub fn new(settings: AutoupdateSettings) -> Self {
        Self(settings)
    }
}

impl From<UpdaterStatusEvent> for CliEvent {
    fn from(event: UpdaterStatusEvent) -> Self {
        CliEvent::UpdaterStatus(Box::new(event))
    }
}

impl From<UpdaterChangelogEvent> for CliEvent {
    fn from(event: UpdaterChangelogEvent) -> Self {
        CliEvent::UpdaterChangelog(event)
    }
}

impl From<UpdaterAutoupdateEvent> for CliEvent {
    fn from(event: UpdaterAutoupdateEvent) -> Self {
        CliEvent::UpdaterAutoupdate(event)
    }
}

impl fmt::Display for UpdaterStatusEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = &self.0;
        let mut fields = Vec::new();

        if let Some(install) = &status.install {
            fields.extend(prefixed("install", install_fields(install)));
        }

        if let Some(check) = &status.check {
            fields.extend(prefixed("check", check_fields(check)));
        }

        write_fields(f, &fields)
    }
}

impl fmt::Display for UpdaterChangelogEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.changelog)
    }
}

impl fmt::Display for UpdaterAutoupdateEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let settings = &self.0;
        let mut fields = Vec::new();

        if let Some(is_enabled) = settings.is_enabled {
            fields.push(field("enabled", is_enabled));
        }

        if let Some(interval_start) = &settings.interval_start {
            fields.push(field("interval start", interval_start));
        }

        if let Some(interval_end) = &settings.interval_end {
            fields.push(field("interval end", interval_end));
        }

        write_fields(f, &fields)
    }
}

fn install_fields(install: &InstallStatus) -> Vec<Field> {
    let mut fields = Vec::new();

    if let Some(is_allowed) = install.is_allowed {
        fields.push(field("allowed", is_allowed));
    }

    if let Some(event) = &install.event {
        fields.push(field("event", update_event_label(event)));
    }

    if let Some(action) = &install.action {
        fields.push(field("action", update_action_label(action)));
    }

    if let Some(status) = &install.status {
        fields.push(field("status", install_result_label(status)));
    }

    if let Some(detail) = &install.detail
        && !detail.is_empty()
    {
        fields.push(field("detail", detail));
    }

    if let Some(download) = &install.download {
        fields.extend(prefixed("download", download_fields(download)));
    }

    fields
}

fn download_fields(download: &DownloadProgress) -> Vec<Field> {
    let mut fields = Vec::new();

    if let Some(speed) = download.speed_bytes_per_sec {
        fields.push(field("speed bytes per sec", speed));
    }

    if let Some(received) = download.received_bytes {
        fields.push(field("received bytes", received));
    }

    if let Some(total) = download.total_bytes {
        fields.push(field("total bytes", total));
    }

    fields
}

fn check_fields(check: &CheckStatus) -> Vec<Field> {
    let mut fields = Vec::new();

    if let Some(version) = check.available_version() {
        fields.push(field("available version", version));
    }

    if let Some(event) = &check.event {
        fields.push(field("event", check_event_label(event)));
    }

    if let Some(status) = &check.status {
        fields.push(field("status", check_result_label(status)));
    }

    fields
}

fn update_event_label(event: &UpdateEvent) -> &str {
    match event {
        UpdateEvent::SessionStart => "session_start",
        UpdateEvent::SessionStop => "session_stop",
        UpdateEvent::ActionBegin => "action_begin",
        UpdateEvent::ActionDone => "action_done",
        UpdateEvent::DetailChange => "detail_change",
        UpdateEvent::ActionProgress => "action_progress",
        UpdateEvent::None => "none",
        UpdateEvent::Unknown(event) => event,
    }
}

fn update_action_label(action: &UpdateAction) -> &str {
    match action {
        UpdateAction::Download => "download",
        UpdateAction::ShaVerification => "sha_verification",
        UpdateAction::Unpack => "unpack",
        UpdateAction::Prepare => "prepare",
        UpdateAction::Apply => "apply",
        UpdateAction::None => "none",
        UpdateAction::Unknown(action) => action,
    }
}

fn install_result_label(status: &InstallResult) -> &str {
    match status {
        InstallResult::Ok => "ok",
        InstallResult::BatteryLow => "battery_low",
        InstallResult::Busy => "busy",
        InstallResult::DownloadFailure => "download_failure",
        InstallResult::DownloadAbort => "download_abort",
        InstallResult::ShaMismatch => "sha_mismatch",
        InstallResult::UnpackStagingDirFailure => "unpack_staging_dir_failure",
        InstallResult::UnpackArchiveOpenFailure => "unpack_archive_open_failure",
        InstallResult::UnpackArchiveUnpackFailure => "unpack_archive_unpack_failure",
        InstallResult::InstallManifestNotFound => "install_manifest_not_found",
        InstallResult::InstallManifestInvalid => "install_manifest_invalid",
        InstallResult::InstallSessionConfigFailure => "install_session_config_failure",
        InstallResult::InstallPointerSetupFailure => "install_pointer_setup_failure",
        InstallResult::UnknownFailure => "unknown_failure",
        InstallResult::Unknown(status) => status,
    }
}

fn check_event_label(event: &CheckEvent) -> &str {
    match event {
        CheckEvent::Start => "start",
        CheckEvent::Stop => "stop",
        CheckEvent::None => "none",
        CheckEvent::Unknown(event) => event,
    }
}

fn check_result_label(status: &CheckResult) -> &str {
    match status {
        CheckResult::Available => "available",
        CheckResult::NotAvailable => "not_available",
        CheckResult::Failure => "failure",
        CheckResult::None => "none",
        CheckResult::Unknown(status) => status,
    }
}

#[cfg(test)]
mod tests {
    use busylib::types::time_of_day::TimeOfDay;

    use super::*;

    #[test]
    fn prefixes_the_install_and_check_sections() {
        let event = UpdaterStatusEvent::new(UpdateStatus {
            install: Some(InstallStatus {
                is_allowed: Some(true),
                event: Some(UpdateEvent::None),
                action: Some(UpdateAction::None),
                status: Some(InstallResult::Ok),
                detail: Some(String::new()),
                download: None,
            }),
            check: Some(CheckStatus {
                available_version: Some(String::new()),
                event: Some(CheckEvent::None),
                status: Some(CheckResult::NotAvailable),
            }),
        });

        assert_eq!(
            event.to_string(),
            "install allowed: true\n\
             install event: none\n\
             install action: none\n\
             install status: ok\n\
             check event: none\n\
             check status: not_available"
        );
    }

    #[test]
    fn renders_the_autoupdate_window() {
        let event = UpdaterAutoupdateEvent::new(AutoupdateSettings {
            is_enabled: Some(true),
            interval_start: Some(TimeOfDay::new("02:00").unwrap()),
            interval_end: Some(TimeOfDay::new("05:00").unwrap()),
        });

        assert_eq!(
            event.to_string(),
            "enabled: true\ninterval start: 02:00\ninterval end: 05:00"
        );
    }
}
