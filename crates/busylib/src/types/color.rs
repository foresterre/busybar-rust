//! Colors

use std::fmt;
use std::str::FromStr;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::invalid_value::InvalidValue;
use crate::types::try_into_value::TryIntoValue;

/// RGBA color, serialized as `#RRGGBBAA`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const BLACK: Self = Self::rgb(0x00, 0x00, 0x00);
    pub const WHITE: Self = Self::rgb(0xff, 0xff, 0xff);
    pub const RED: Self = Self::rgb(0xff, 0x00, 0x00);
    pub const GREEN: Self = Self::rgb(0x00, 0xff, 0x00);
    pub const BLUE: Self = Self::rgb(0x00, 0x00, 0xff);
    pub const TRANSPARENT: Self = Self::rgba(0x00, 0x00, 0x00, 0x00);

    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::rgba(red, green, blue, 0xff)
    }

    pub fn parse(value: &str) -> Result<Self, InvalidValue> {
        let invalid = || InvalidValue::new("color", value, "a color in #RRGGBBAA format");

        let hex = value.strip_prefix('#').ok_or_else(invalid)?;

        if hex.len() != 8 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(invalid());
        }

        let channel = |range: std::ops::Range<usize>| u8::from_str_radix(&hex[range], 16);

        match (channel(0..2), channel(2..4), channel(4..6), channel(6..8)) {
            (Ok(red), Ok(green), Ok(blue), Ok(alpha)) => Ok(Self::rgba(red, green, blue, alpha)),
            _ => Err(invalid()),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

impl FromStr for Color {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryIntoValue<Color> for &str {
    fn try_into_value(self) -> Result<Color, InvalidValue> {
        Color::parse(self)
    }
}

impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}
