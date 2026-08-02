use std::path::PathBuf;

use busylib::types::app_name::AppName;
use busylib::types::brightness::Brightness;
use busylib::types::color::Color;
use busylib::types::priority::Priority;
use busylib::types::text::Text;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::values::{AlignArg, FontArg, ScreenArg};

#[derive(Debug, Subcommand)]
pub enum DisplayCommand {
    /// Draw a single line of text
    Text {
        /// Printable ASCII text
        #[arg(value_name = "TEXT")]
        text: Text,

        /// Application name
        #[arg(long, short = 'a', value_name = "NAME")]
        app: AppName,

        /// Font to draw the text in
        #[arg(long, value_enum, default_value_t = FontArg::Normal)]
        font: FontArg,

        /// Anchor point of the text
        #[arg(long, value_enum)]
        align: Option<AlignArg>,

        /// Text color in #RRGGBBAA format
        #[arg(long, value_name = "COLOR")]
        color: Option<Color>,

        /// X coordinate of the anchor point
        #[arg(long, allow_negative_numbers = true, default_value_t = 0)]
        x: i16,

        /// Y coordinate of the anchor point
        #[arg(long, allow_negative_numbers = true, default_value_t = 0)]
        y: i16,

        /// Screen to draw on
        #[arg(long, value_enum, default_value_t = ScreenArg::Front)]
        screen: ScreenArg,

        /// Seconds until the text is hidden
        #[arg(long, value_name = "SECONDS")]
        timeout: Option<u32>,

        /// Draw priority between 1 and 100
        #[arg(long, value_name = "PRIORITY")]
        priority: Option<Priority>,

        /// Blink the status LED in this color
        #[arg(long, value_name = "COLOR")]
        led: Option<Color>,
    },

    /// Draw elements from a JSON payload
    Draw {
        /// JSON draw request, or - for stdin
        #[arg(value_name = "FILE")]
        file: PathBuf,
    },

    /// Clear drawn elements
    Clear {
        /// Only clear the elements of this application
        #[arg(long, short = 'a', value_name = "NAME")]
        app: Option<AppName>,
    },

    /// Show the display brightness
    Brightness,

    /// Set the display brightness
    SetBrightness {
        /// Percentage between 0 and 100, or auto
        #[arg(value_name = "VALUE")]
        value: Brightness,
    },

    /// Capture a single frame of a screen
    Frame {
        /// Screen to capture
        #[arg(value_enum, default_value_t = ScreenArg::Front)]
        screen: ScreenArg,

        /// Write the bitmap here instead of stdout
        #[arg(long, short = 'O', value_name = "FILE")]
        output: Option<PathBuf>,
    },
}

impl DisplayCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
