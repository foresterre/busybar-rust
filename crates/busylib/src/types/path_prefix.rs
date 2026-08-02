//! Path prefixes the API is mounted under
//!
//! ... because the [spec](https://api.busy.app/busybar/docs?urls.primaryName=1.1.1) uses `/busybar/<...>`
//! while a device connected via USB is at `/api/<...>`, ugh :).

use crate::types::invalid_value::InvalidValue;
use crate::types::validate;

/// Path the API is mounted under, without the surrounding slashes
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathPrefix(&'static str);

impl PathPrefix {
    pub fn new(value: &'static str) -> Result<Self, InvalidValue> {
        if validate::path_prefix(value) {
            Ok(Self(value))
        } else {
            Err(InvalidValue::new(
                "path prefix",
                value,
                "`/` separated, non-empty segments of [a-zA-Z0-9._-], without leading or trailing `/`",
            ))
        }
    }

    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Debug for PathPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PathPrefix({:?})", self.0)
    }
}

impl std::fmt::Display for PathPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl AsRef<str> for PathPrefix {
    fn as_ref(&self) -> &str {
        self.0
    }
}

impl TryFrom<&'static str> for PathPrefix {
    type Error = InvalidValue;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
