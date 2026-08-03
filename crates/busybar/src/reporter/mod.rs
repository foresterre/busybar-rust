mod events;
mod human;
mod json;
mod output;

use std::sync::Arc;

use storyteller::{
    ChannelEventListener, ChannelHandlerGuard, ChannelReporter, EventListener, EventReporter,
    HandlerGuard, event_channel,
};

use crate::reporter::human::HumanHandler;
use crate::reporter::json::JsonHandler;
use crate::types::output_format::OutputFormatArg;

pub use crate::reporter::events::{
    AccountBackendEvent, AccountInfoEvent, AccountStatusEvent, BleStatusEvent, BusyProfileEvent,
    BusySnapshotEvent, CliEvent, FramePayload, OkEvent, Payload, SettingsAccessEvent,
    SettingsBrightnessEvent, SettingsNameEvent, SettingsVolumeEvent, SmartHomePairingEvent,
    SmartHomeStartPairingEvent, SmartHomeSwitchEvent, StorageListEvent, StorageReadEvent,
    StorageStatusEvent, StreamingScreenEvent, StreamingStatusEvent, SystemLogDumpEvent,
    SystemStatusDeviceEvent, SystemStatusEvent, SystemStatusFirmwareEvent, SystemStatusPowerEvent,
    SystemStatusSystemEvent, SystemTransportEvent, SystemVersionEvent, TimeNowEvent,
    TimeTimezoneEvent, TimeTzlistEvent, UnsupportedEvent, UpdaterAutoupdateEvent,
    UpdaterChangelogEvent, UpdaterStatusEvent, WifiStatusEvent,
};
pub use crate::reporter::output::{Output, OutputError};

#[derive(Debug, thiserror::Error)]
pub enum ReporterError {
    #[error("the output handler disconnected before the event was reported")]
    Disconnected,

    #[error("the output handler panicked")]
    Panicked,

    #[error(transparent)]
    Output(#[from] OutputError),
}

pub struct Reporter {
    reporter: ChannelReporter<CliEvent>,
    guard: ChannelHandlerGuard,
    output: Arc<Output>,
}

impl Reporter {
    pub fn new(format: OutputFormatArg) -> Self {
        Self::with_output(format, Arc::new(Output::stdout()))
    }

    pub fn with_output(format: OutputFormatArg, output: Arc<Output>) -> Self {
        let (sender, receiver) = event_channel::<CliEvent>();
        let listener = ChannelEventListener::new(receiver);

        let guard = match format {
            OutputFormatArg::Text => {
                listener.run_handler(Arc::new(HumanHandler::new(Arc::clone(&output))))
            }
            OutputFormatArg::Json => {
                listener.run_handler(Arc::new(JsonHandler::new(Arc::clone(&output))))
            }
        };

        Self {
            reporter: ChannelReporter::new(sender),
            guard,
            output,
        }
    }

    pub fn report(&self, event: impl Into<CliEvent>) -> Result<(), ReporterError> {
        self.reporter
            .report_event(event)
            .map_err(|_| ReporterError::Disconnected)
    }

    pub fn finish(self) -> Result<(), ReporterError> {
        let Self {
            reporter,
            guard,
            output,
        } = self;

        let token = reporter
            .disconnect()
            .map_err(|_| ReporterError::Disconnected)?;
        guard.join(token).map_err(|()| ReporterError::Panicked)?;

        match output.take_error() {
            Some(error) => Err(ReporterError::Output(error)),
            None => Ok(()),
        }
    }
}

impl std::fmt::Debug for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Reporter")
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::sync::Mutex;

    use busylib::model::wifi::{StatusResponse, WifiState};

    use super::*;

    #[derive(Clone, Default)]
    struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

    impl SharedBuffer {
        fn contents(&self) -> Vec<u8> {
            self.0.lock().unwrap().clone()
        }
    }

    impl Write for SharedBuffer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn human_reporter_finishes_without_events() {
        let buffer = SharedBuffer::default();
        let output = Arc::new(Output::new(buffer.clone()));
        let reporter = Reporter::with_output(OutputFormatArg::Text, output);

        reporter.finish().unwrap();

        assert!(buffer.contents().is_empty());
    }

    #[test]
    fn json_reporter_finishes_without_events() {
        let buffer = SharedBuffer::default();
        let output = Arc::new(Output::new(buffer.clone()));
        let reporter = Reporter::with_output(OutputFormatArg::Json, output);

        reporter.finish().unwrap();

        assert!(buffer.contents().is_empty());
    }

    fn sample_event() -> CliEvent {
        CliEvent::from(WifiStatusEvent::new(StatusResponse {
            state: WifiState::Disconnected,
            ssid: None,
            bssid: None,
            channel: None,
            rssi: None,
            security: None,
            ip_config: None,
        }))
    }

    fn render(format: OutputFormatArg, event: CliEvent) -> String {
        let buffer = SharedBuffer::default();
        let output = Arc::new(Output::new(buffer.clone()));
        let reporter = Reporter::with_output(format, output);

        reporter.report(event).unwrap();
        reporter.finish().unwrap();

        String::from_utf8(buffer.contents()).unwrap()
    }

    #[test]
    fn human_reporter_writes_events_as_they_display() {
        let event = sample_event();

        assert_eq!(
            render(OutputFormatArg::Text, event.clone()),
            format!("{event}\n")
        );
    }

    #[test]
    fn json_reporter_writes_events_as_a_line_of_json() {
        let event = sample_event();

        assert_eq!(
            render(OutputFormatArg::Json, event.clone()),
            format!("{}\n", serde_json::to_string(&event).unwrap())
        );
    }
}
