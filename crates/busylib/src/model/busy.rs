use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusySnapshot {
    pub snapshot: Snapshot,
    pub snapshot_timestamp_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub state: TimerState,
    pub busy_bar_settings: BusyBarSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TimerState {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "INFINITE")]
    Infinite { card_id: String, is_paused: bool },
    #[serde(rename = "SIMPLE")]
    Simple {
        card_id: String,
        time_left_ms: u64,
        is_paused: bool,
    },
    #[serde(rename = "INTERVAL")]
    Interval {
        card_id: String,
        current_interval: u32,
        current_interval_time_total_ms: u64,
        current_interval_time_left_ms: u64,
        is_paused: bool,
        #[serde(with = "tagged_interval_settings")]
        interval_settings: IntervalSettings,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusyProfile {
    pub id: String,
    pub title: String,
    pub sort_order: i32,
    pub timer_settings: TimerSettings,
    pub busy_bar_settings: BusyBarSettings,
    pub profile_timestamp_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusyProfileSlot {
    Busy,
    Custom,
}

impl BusyProfileSlot {
    pub fn as_str(self) -> &'static str {
        match self {
            BusyProfileSlot::Busy => "busy",
            BusyProfileSlot::Custom => "custom",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TimerSettings {
    #[serde(rename = "INFINITE")]
    Infinite,
    #[serde(rename = "SIMPLE")]
    Simple { total_time_ms: u64 },
    #[serde(rename = "INTERVAL")]
    Interval {
        interval_work_ms: u64,
        interval_rest_ms: u64,
        interval_work_cycles_count: u32,
        is_autostart_enabled: bool,
    },
}

impl TimerSettings {
    pub fn interval(settings: IntervalSettings) -> Self {
        Self::Interval {
            interval_work_ms: settings.interval_work_ms,
            interval_rest_ms: settings.interval_rest_ms,
            interval_work_cycles_count: settings.interval_work_cycles_count,
            is_autostart_enabled: settings.is_autostart_enabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntervalSettings {
    pub interval_work_ms: u64,
    pub interval_rest_ms: u64,
    pub interval_work_cycles_count: u32,
    pub is_autostart_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusyBarSettings {
    pub theme: String,
    pub show_work_phase_only: bool,
    pub trigger_smart_home: bool,
}

// The device tags nested interval settings with the same `type` discriminator as
// standalone timer settings, so the tag is added and checked on the way through.
mod tagged_interval_settings {
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::{IntervalSettings, TimerSettings};

    pub fn serialize<S: Serializer>(
        value: &IntervalSettings,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        TimerSettings::interval(*value).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<IntervalSettings, D::Error> {
        match TimerSettings::deserialize(deserializer)? {
            TimerSettings::Interval {
                interval_work_ms,
                interval_rest_ms,
                interval_work_cycles_count,
                is_autostart_enabled,
            } => Ok(IntervalSettings {
                interval_work_ms,
                interval_rest_ms,
                interval_work_cycles_count,
                is_autostart_enabled,
            }),
            _ => Err(D::Error::custom("expected INTERVAL timer settings")),
        }
    }
}
