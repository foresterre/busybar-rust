use std::fmt;

use clap::ValueEnum;

use busybar_render::ImageFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ImageFormatArg {
    Png,
    Bmp,
    Jpg,
    Raw,
}

impl ImageFormatArg {
    pub fn image_format(self) -> Option<ImageFormat> {
        match self {
            ImageFormatArg::Png => Some(ImageFormat::Png),
            ImageFormatArg::Bmp => Some(ImageFormat::Bmp),
            ImageFormatArg::Jpg => Some(ImageFormat::Jpeg),
            ImageFormatArg::Raw => None,
        }
    }
}

impl fmt::Display for ImageFormatArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageFormatArg::Png => f.write_str("png"),
            ImageFormatArg::Bmp => f.write_str("bmp"),
            ImageFormatArg::Jpg => f.write_str("jpg"),
            ImageFormatArg::Raw => f.write_str("raw"),
        }
    }
}
