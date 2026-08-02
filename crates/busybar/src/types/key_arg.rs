use busylib::model::input::Key;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum KeyArg {
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

impl From<KeyArg> for Key {
    fn from(key: KeyArg) -> Self {
        match key {
            KeyArg::Up => Key::Up,
            KeyArg::Down => Key::Down,
            KeyArg::Ok => Key::Ok,
            KeyArg::Back => Key::Back,
            KeyArg::Start => Key::Start,
            KeyArg::Busy => Key::Busy,
            KeyArg::Custom => Key::Custom,
            KeyArg::Off => Key::Off,
            KeyArg::Apps => Key::Apps,
            KeyArg::Settings => Key::Settings,
        }
    }
}
