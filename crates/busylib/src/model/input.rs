use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
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

impl Key {
    pub fn as_str(self) -> &'static str {
        match self {
            Key::Up => "up",
            Key::Down => "down",
            Key::Ok => "ok",
            Key::Back => "back",
            Key::Start => "start",
            Key::Busy => "busy",
            Key::Custom => "custom",
            Key::Off => "off",
            Key::Apps => "apps",
            Key::Settings => "settings",
        }
    }
}
