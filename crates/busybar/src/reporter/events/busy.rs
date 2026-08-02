use std::fmt;

use busylib::model::busy::{
    BusyBarSettings, BusyProfile, BusySnapshot, BusyTimerIntervalSettings, TimerSettings,
    TimerState,
};
use serde::Serialize;

use crate::reporter::events::CliEvent;
use crate::reporter::events::fields::{Field, field, prefixed, write_fields};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BusySnapshotEvent(BusySnapshot);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BusyProfileEvent(BusyProfile);

impl BusySnapshotEvent {
    pub fn new(snapshot: BusySnapshot) -> Self {
        Self(snapshot)
    }
}

impl BusyProfileEvent {
    pub fn new(profile: BusyProfile) -> Self {
        Self(profile)
    }
}

impl From<BusySnapshotEvent> for CliEvent {
    fn from(event: BusySnapshotEvent) -> Self {
        CliEvent::BusySnapshot(Box::new(event))
    }
}

impl From<BusyProfileEvent> for CliEvent {
    fn from(event: BusyProfileEvent) -> Self {
        CliEvent::BusyProfile(Box::new(event))
    }
}

impl fmt::Display for BusySnapshotEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let snapshot = &self.0;
        let mut fields = state_fields(&snapshot.snapshot.state);

        fields.extend(prefixed(
            "settings",
            bar_settings_fields(&snapshot.snapshot.busy_bar_settings),
        ));
        fields.push(field(
            "snapshot timestamp ms",
            snapshot.snapshot_timestamp_ms,
        ));

        write_fields(f, &fields)
    }
}

impl fmt::Display for BusyProfileEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let profile = &self.0;
        let mut fields = vec![
            field("id", &profile.id),
            field("title", &profile.title),
            field("sort order", profile.sort_order),
        ];

        fields.extend(prefixed(
            "timer",
            timer_settings_fields(&profile.timer_settings),
        ));
        fields.extend(prefixed(
            "settings",
            bar_settings_fields(&profile.busy_bar_settings),
        ));
        fields.push(field("profile timestamp ms", profile.profile_timestamp_ms));

        write_fields(f, &fields)
    }
}

fn state_fields(state: &TimerState) -> Vec<Field> {
    match state {
        TimerState::NotStarted => vec![field("type", "NOT_STARTED")],
        TimerState::Infinite { card_id, is_paused } => vec![
            field("type", "INFINITE"),
            field("card id", card_id),
            field("paused", is_paused),
        ],
        TimerState::Simple {
            card_id,
            time_left_ms,
            is_paused,
        } => vec![
            field("type", "SIMPLE"),
            field("card id", card_id),
            field("time left ms", time_left_ms),
            field("paused", is_paused),
        ],
        TimerState::Interval {
            card_id,
            current_interval,
            current_interval_time_total_ms,
            current_interval_time_left_ms,
            is_paused,
            interval_settings,
        } => {
            let mut fields = vec![
                field("type", "INTERVAL"),
                field("card id", card_id),
                field("current interval", current_interval),
                field(
                    "current interval time total ms",
                    current_interval_time_total_ms,
                ),
                field(
                    "current interval time left ms",
                    current_interval_time_left_ms,
                ),
                field("paused", is_paused),
            ];
            fields.extend(prefixed("interval", interval_fields(interval_settings)));
            fields
        }
    }
}

fn timer_settings_fields(settings: &TimerSettings) -> Vec<Field> {
    match settings {
        TimerSettings::Infinite => vec![field("type", "INFINITE")],
        TimerSettings::Simple { total_time_ms } => vec![
            field("type", "SIMPLE"),
            field("total time ms", total_time_ms),
        ],
        TimerSettings::Interval {
            interval_work_ms,
            interval_rest_ms,
            interval_work_cycles_count,
            is_autostart_enabled,
        } => vec![
            field("type", "INTERVAL"),
            field("work ms", interval_work_ms),
            field("rest ms", interval_rest_ms),
            field("work cycles count", interval_work_cycles_count),
            field("autostart enabled", is_autostart_enabled),
        ],
    }
}

fn interval_fields(settings: &BusyTimerIntervalSettings) -> Vec<Field> {
    vec![
        field("work ms", settings.interval_work_ms),
        field("rest ms", settings.interval_rest_ms),
        field("work cycles count", settings.interval_work_cycles_count),
        field("autostart enabled", settings.is_autostart_enabled),
    ]
}

fn bar_settings_fields(settings: &BusyBarSettings) -> Vec<Field> {
    vec![
        field("theme", &settings.theme),
        field("show work phase only", settings.show_work_phase_only),
        field("trigger smart home", settings.trigger_smart_home),
    ]
}

#[cfg(test)]
mod tests {
    use busylib::model::busy::Snapshot;

    use super::*;

    fn bar_settings() -> BusyBarSettings {
        BusyBarSettings {
            theme: "default".to_string(),
            show_work_phase_only: false,
            trigger_smart_home: true,
        }
    }

    #[test]
    fn renders_the_running_timer_state() {
        let event = BusySnapshotEvent::new(BusySnapshot {
            snapshot: Snapshot {
                state: TimerState::Infinite {
                    card_id: "00000000-0000-0000-0000-000000000001".to_string(),
                    is_paused: true,
                },
                busy_bar_settings: bar_settings(),
            },
            snapshot_timestamp_ms: 1761582532251,
        });

        assert_eq!(
            event.to_string(),
            "type: INFINITE\n\
             card id: 00000000-0000-0000-0000-000000000001\n\
             paused: true\n\
             settings theme: default\n\
             settings show work phase only: false\n\
             settings trigger smart home: true\n\
             snapshot timestamp ms: 1761582532251"
        );
    }

    #[test]
    fn prefixes_the_timer_settings_of_a_profile() {
        let event = BusyProfileEvent::new(BusyProfile {
            id: "00000000-0000-0000-0000-000000000000".to_string(),
            title: "BUSY".to_string(),
            sort_order: 0,
            timer_settings: TimerSettings::Simple {
                total_time_ms: 1_500_000,
            },
            busy_bar_settings: bar_settings(),
            profile_timestamp_ms: 1761582532251,
        });

        assert_eq!(
            event.to_string(),
            "id: 00000000-0000-0000-0000-000000000000\n\
             title: BUSY\n\
             sort order: 0\n\
             timer type: SIMPLE\n\
             timer total time ms: 1500000\n\
             settings theme: default\n\
             settings show work phase only: false\n\
             settings trigger smart home: true\n\
             profile timestamp ms: 1761582532251"
        );
    }
}
