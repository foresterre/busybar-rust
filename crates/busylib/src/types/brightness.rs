use std::fmt;
use std::str::FromStr;

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::types::invalid_value::InvalidValue;

/// Display brightness, either automatic or a fixed percentage.
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
