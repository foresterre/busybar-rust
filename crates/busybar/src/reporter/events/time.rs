use std::fmt;

use busylib::model::time::TimezoneInfo;
use busylib::types::timestamp::Timestamp;
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::fields::{field, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TimeNowEvent {
    timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TimeTimezoneEvent(TimezoneInfo);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TimeTzlistEvent {
    list: Vec<TimezoneInfo>,
}

impl TimeNowEvent {
    pub fn new(timestamp: Timestamp) -> Self {
        Self { timestamp }
    }
}

impl TimeTimezoneEvent {
    pub fn new(timezone: TimezoneInfo) -> Self {
        Self(timezone)
    }
}

impl TimeTzlistEvent {
    pub fn new(list: Vec<TimezoneInfo>) -> Self {
        Self { list }
    }
}

impl From<TimeNowEvent> for CliEvent {
    fn from(event: TimeNowEvent) -> Self {
        CliEvent::TimeNow(event)
    }
}

impl From<TimeTimezoneEvent> for CliEvent {
    fn from(event: TimeTimezoneEvent) -> Self {
        CliEvent::TimeTimezone(event)
    }
}

impl From<TimeTzlistEvent> for CliEvent {
    fn from(event: TimeTzlistEvent) -> Self {
        CliEvent::TimeTzlist(event)
    }
}

impl fmt::Display for TimeNowEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.timestamp.as_str())
    }
}

impl fmt::Display for TimeTimezoneEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timezone = &self.0;
        let fields = vec![
            field("name", &timezone.name),
            field("offset", &timezone.offset),
            field("abbr", &timezone.abbr),
        ];

        write_fields(f, &fields)
    }
}

impl fmt::Display for TimeTzlistEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, timezone) in self.list.iter().enumerate() {
            if index > 0 {
                f.write_str("\n")?;
            }

            write!(f, "{} {} {}", timezone.name, timezone.offset, timezone.abbr)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn amsterdam() -> TimezoneInfo {
        TimezoneInfo {
            name: "Amsterdam".to_string(),
            offset: "+02:00".to_string(),
            abbr: "CEST".to_string(),
        }
    }

    #[test]
    fn renders_the_clock_bare() {
        let event = TimeNowEvent::new(Timestamp::new("2026-08-02T05:23:32+02:00").unwrap());

        assert_eq!(event.to_string(), "2026-08-02T05:23:32+02:00");
        assert_eq!(
            serde_json::to_value(CliEvent::from(event)).unwrap(),
            json!({"event": "time_now", "timestamp": "2026-08-02T05:23:32+02:00"})
        );
    }

    #[test]
    fn renders_a_time_zone_and_a_list_of_them() {
        assert_eq!(
            TimeTimezoneEvent::new(amsterdam()).to_string(),
            "name: Amsterdam\noffset: +02:00\nabbr: CEST"
        );
        assert_eq!(
            TimeTzlistEvent::new(vec![amsterdam(), amsterdam()]).to_string(),
            "Amsterdam +02:00 CEST\nAmsterdam +02:00 CEST"
        );
    }
}
