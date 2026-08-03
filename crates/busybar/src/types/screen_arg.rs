use std::fmt;

use busylib::model::assets::Screen;
use busylib::proto::bsb_frame::Screen as FrameScreen;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ScreenArg {
    Front,
    Back,
}

impl fmt::Display for ScreenArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScreenArg::Front => f.write_str("front"),
            ScreenArg::Back => f.write_str("back"),
        }
    }
}

impl From<ScreenArg> for Screen {
    fn from(screen: ScreenArg) -> Self {
        match screen {
            ScreenArg::Front => Screen::Front,
            ScreenArg::Back => Screen::Back,
        }
    }
}

impl From<ScreenArg> for FrameScreen {
    fn from(screen: ScreenArg) -> Self {
        match screen {
            ScreenArg::Front => FrameScreen::Front,
            ScreenArg::Back => FrameScreen::Back,
        }
    }
}
