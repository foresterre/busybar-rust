use std::fmt;
use std::str::FromStr;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid {label} `{value}`: expected {expectation}")]
pub struct InvalidValue {
    label: &'static str,
    value: String,
    expectation: &'static str,
}

impl InvalidValue {
    fn new(label: &'static str, value: impl Into<String>, expectation: &'static str) -> Self {
        Self {
            label,
            value: value.into(),
            expectation,
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn expectation(&self) -> &'static str {
        self.expectation
    }
}

pub trait TryIntoValue<T> {
    fn try_into_value(self) -> Result<T, InvalidValue>;
}

impl<T> TryIntoValue<T> for T {
    fn try_into_value(self) -> Result<T, InvalidValue> {
        Ok(self)
    }
}

mod validate {
    pub fn name(value: &str) -> bool {
        !value.is_empty() && value.chars().all(is_name_char)
    }

    pub fn asset_path(value: &str) -> bool {
        !value.is_empty()
            && value
                .chars()
                .all(|character| is_name_char(character) || character == '/')
    }

    pub fn stock_path(value: &str) -> bool {
        match value.strip_prefix("shared/") {
            Some(rest) => {
                !rest.is_empty()
                    && rest.chars().all(|character| {
                        character.is_ascii_lowercase()
                            || character.is_ascii_digit()
                            || matches!(character, '_' | '.')
                    })
            }
            None => false,
        }
    }

    pub fn storage_path(value: &str) -> bool {
        let Some(rest) = value.strip_prefix("/ext") else {
            return false;
        };

        if rest.is_empty() {
            return true;
        }

        if !rest.starts_with('/') {
            return false;
        }

        rest.split('/')
            .skip(1)
            .all(|segment| segment.chars().all(is_name_char))
    }

    pub fn device_name(value: &str) -> bool {
        let length = value.chars().count();
        (1..=20).contains(&length)
            && value.chars().all(|character| {
                character.is_ascii_alphanumeric()
                    || matches!(
                        character,
                        ' ' | '!'
                            | '('
                            | ')'
                            | '_'
                            | '='
                            | '+'
                            | ';'
                            | ':'
                            | ','
                            | '.'
                            | '?'
                            | '\''
                            | '|'
                            | '@'
                            | '#'
                            | '$'
                            | '%'
                            | '^'
                            | '&'
                            | '*'
                            | '['
                            | ']'
                            | '{'
                            | '}'
                            | '/'
                            | '\\'
                            | '"'
                            | '<'
                            | '>'
                            | '-'
                    )
            })
    }

    pub fn printable_ascii(value: &str) -> bool {
        !value.is_empty() && value.bytes().all(|byte| (0x20..=0x7e).contains(&byte))
    }

    pub fn log_name(value: &str) -> bool {
        !value.is_empty()
            && value.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '_' | '-')
            })
    }

    pub fn timezone_name(value: &str) -> bool {
        let mut characters = value.chars();
        let Some(first) = characters.next() else {
            return false;
        };
        first.is_ascii_alphabetic()
            && value.chars().count() <= 51
            && characters.all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, ' ' | '_' | '+' | '-')
            })
    }

    pub fn time_of_day(value: &str) -> bool {
        let bytes = value.as_bytes();
        if bytes.len() != 5 || bytes[2] != b':' {
            return false;
        }
        matches!(
            (two_digits(&bytes[..2]), two_digits(&bytes[3..])),
            (Some(hours), Some(minutes)) if hours <= 23 && minutes <= 59
        )
    }

    pub fn access_key(value: &str) -> bool {
        (4..=10).contains(&value.len()) && value.bytes().all(|byte| byte.is_ascii_digit())
    }

    pub fn timestamp(value: &str) -> bool {
        let bytes = value.as_bytes();

        if bytes.len() < 20 || &bytes[..2] != b"20" {
            return false;
        }

        if two_digits(&bytes[2..4]).is_none() {
            return false;
        }

        if bytes[4] != b'-'
            || bytes[7] != b'-'
            || bytes[10] != b'T'
            || bytes[13] != b':'
            || bytes[16] != b':'
        {
            return false;
        }

        let (Some(month), Some(day), Some(hour), Some(minute), Some(second)) = (
            two_digits(&bytes[5..7]),
            two_digits(&bytes[8..10]),
            two_digits(&bytes[11..13]),
            two_digits(&bytes[14..16]),
            two_digits(&bytes[17..19]),
        ) else {
            return false;
        };

        if !(1..=12).contains(&month)
            || !(1..=31).contains(&day)
            || hour > 23
            || minute > 59
            || second > 59
        {
            return false;
        }

        offset(&bytes[19..])
    }

    fn offset(bytes: &[u8]) -> bool {
        if bytes == b"Z" {
            return true;
        }

        if !matches!(bytes.first(), Some(b'+' | b'-')) || bytes.len() < 3 {
            return false;
        }

        match two_digits(&bytes[1..3]) {
            Some(hours) if hours <= 23 => {}
            _ => return false,
        }

        match bytes.len() {
            3 => true,
            5 => two_digits(&bytes[3..5]).is_some_and(|minutes| minutes <= 59),
            6 => bytes[3] == b':' && two_digits(&bytes[4..6]).is_some_and(|minutes| minutes <= 59),
            _ => false,
        }
    }

    fn two_digits(bytes: &[u8]) -> Option<u8> {
        match bytes {
            [tens, ones] if tens.is_ascii_digit() && ones.is_ascii_digit() => {
                Some((tens - b'0') * 10 + (ones - b'0'))
            }
            _ => None,
        }
    }

    fn is_name_char(character: char) -> bool {
        character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-')
    }
}

macro_rules! string_newtype {
    ($name:ident, $label:literal, $expectation:literal, $validate:path) => {
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, InvalidValue> {
                let value = value.into();
                if $validate(&value) {
                    Ok(Self(value))
                } else {
                    Err(InvalidValue::new($label, value, $expectation))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({:?})"), self.0)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl FromStr for $name {
            type Err = InvalidValue;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = InvalidValue;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = InvalidValue;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl TryIntoValue<$name> for &str {
            fn try_into_value(self) -> Result<$name, InvalidValue> {
                $name::new(self)
            }
        }

        impl TryIntoValue<$name> for String {
            fn try_into_value(self) -> Result<$name, InvalidValue> {
                $name::new(self)
            }
        }

        impl TryIntoValue<$name> for &String {
            fn try_into_value(self) -> Result<$name, InvalidValue> {
                $name::new(self.as_str())
            }
        }

        impl TryIntoValue<$name> for &$name {
            fn try_into_value(self) -> Result<$name, InvalidValue> {
                Ok(self.clone())
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::new(value).map_err(D::Error::custom)
            }
        }
    };
}

string_newtype!(
    AppName,
    "application name",
    "one or more of [a-zA-Z0-9._-]",
    validate::name
);
string_newtype!(
    AssetName,
    "asset file name",
    "one or more of [a-zA-Z0-9._-]",
    validate::name
);
string_newtype!(
    AssetPath,
    "asset path",
    "one or more of [a-zA-Z0-9._/-]",
    validate::asset_path
);
string_newtype!(
    StockPath,
    "stock asset path",
    "`shared/` followed by one or more of [a-z0-9_.]",
    validate::stock_path
);
string_newtype!(
    StoragePath,
    "storage path",
    "`/ext` optionally followed by `/` separated segments of [a-zA-Z0-9._-]",
    validate::storage_path
);
string_newtype!(
    DeviceName,
    "device name",
    "1 to 20 letters, digits, spaces or common punctuation (no backtick or tilde)",
    validate::device_name
);
string_newtype!(
    ElementId,
    "element id",
    "one or more of [a-zA-Z0-9._-]",
    validate::name
);
string_newtype!(
    Text,
    "display text",
    "one or more printable ASCII characters",
    validate::printable_ascii
);
string_newtype!(
    LogName,
    "log file name",
    "one or more of [a-zA-Z0-9_-], without extension",
    validate::log_name
);
string_newtype!(
    TimezoneName,
    "time zone name",
    "a letter followed by up to 50 of [A-Za-z0-9 _+-]",
    validate::timezone_name
);
string_newtype!(
    TimeOfDay,
    "time of day",
    "a 24 hour time in HH:MM format",
    validate::time_of_day
);
string_newtype!(
    Timestamp,
    "timestamp",
    "an ISO 8601 timestamp with time zone, such as 2025-10-02T14:30:45+02:00",
    validate::timestamp
);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessKey(String);

impl AccessKey {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidValue> {
        let value = value.into();
        if validate::access_key(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidValue::new(
                "access key",
                "<redacted>",
                "4 to 10 digits",
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for AccessKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AccessKey(<redacted>)")
    }
}

impl FromStr for AccessKey {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryIntoValue<AccessKey> for &str {
    fn try_into_value(self) -> Result<AccessKey, InvalidValue> {
        AccessKey::new(self)
    }
}

impl TryIntoValue<AccessKey> for String {
    fn try_into_value(self) -> Result<AccessKey, InvalidValue> {
        AccessKey::new(self)
    }
}

impl TryIntoValue<AccessKey> for &AccessKey {
    fn try_into_value(self) -> Result<AccessKey, InvalidValue> {
        Ok(self.clone())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Token(String);

impl Token {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidValue> {
        let value = value.into();
        let usable = !value.is_empty() && value.bytes().all(|byte| (0x21..=0x7e).contains(&byte));
        if usable {
            Ok(Self(value))
        } else {
            Err(InvalidValue::new(
                "API token",
                "<redacted>",
                "one or more visible ASCII characters",
            ))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Token(<redacted>)")
    }
}

impl FromStr for Token {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryIntoValue<Token> for &str {
    fn try_into_value(self) -> Result<Token, InvalidValue> {
        Token::new(self)
    }
}

impl TryIntoValue<Token> for String {
    fn try_into_value(self) -> Result<Token, InvalidValue> {
        Token::new(self)
    }
}

impl TryIntoValue<Token> for &Token {
    fn try_into_value(self) -> Result<Token, InvalidValue> {
        Ok(self.clone())
    }
}

macro_rules! percentage_newtype {
    ($name:ident, $label:literal, $expectation:literal, $min:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u8);

        impl $name {
            pub const MIN: Self = Self($min);
            pub const MAX: Self = Self(100);

            pub fn new(percent: u8) -> Result<Self, InvalidValue> {
                if ($min..=100).contains(&percent) {
                    Ok(Self(percent))
                } else {
                    Err(InvalidValue::new($label, percent.to_string(), $expectation))
                }
            }

            pub fn percent(self) -> u8 {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = InvalidValue;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let percent = value
                    .parse::<u8>()
                    .map_err(|_| InvalidValue::new($label, value, $expectation))?;
                Self::new(percent)
            }
        }

        impl TryFrom<u8> for $name {
            type Error = InvalidValue;

            fn try_from(percent: u8) -> Result<Self, Self::Error> {
                Self::new(percent)
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_u8(self.0)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let percent = f64::deserialize(deserializer)?;
                if !(0.0..=100.0).contains(&percent) {
                    return Err(D::Error::custom(InvalidValue::new(
                        $label,
                        percent.to_string(),
                        $expectation,
                    )));
                }
                Self::new(percent.round() as u8).map_err(D::Error::custom)
            }
        }
    };
}

percentage_newtype!(Volume, "volume", "a percentage between 0 and 100", 0);
percentage_newtype!(Opacity, "opacity", "a percentage between 0 and 100", 0);
percentage_newtype!(Priority, "draw priority", "a value between 1 and 100", 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Brightness {
    Auto,
    Level(u8),
}

impl Brightness {
    pub fn level(percent: u8) -> Result<Self, InvalidValue> {
        if percent <= 100 {
            Ok(Self::Level(percent))
        } else {
            Err(InvalidValue::new(
                "brightness",
                percent.to_string(),
                "`auto` or a percentage between 0 and 100",
            ))
        }
    }

    pub fn parse(value: &str) -> Result<Self, InvalidValue> {
        if value == "auto" {
            return Ok(Self::Auto);
        }
        value
            .parse::<u8>()
            .map_err(|_| ())
            .and_then(|percent| if percent <= 100 { Ok(percent) } else { Err(()) })
            .map(Self::Level)
            .map_err(|()| {
                InvalidValue::new(
                    "brightness",
                    value,
                    "`auto` or a percentage between 0 and 100",
                )
            })
    }
}

impl fmt::Display for Brightness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Brightness::Auto => f.write_str("auto"),
            Brightness::Level(percent) => write!(f, "{percent}"),
        }
    }
}

impl FromStr for Brightness {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl Serialize for Brightness {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Brightness {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

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
