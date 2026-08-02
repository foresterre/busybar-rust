//! HTTP API access keys

use std::fmt;
use std::str::FromStr;

use crate::types::invalid_value::InvalidValue;
use crate::types::try_into_value::TryIntoValue;
use crate::types::validate;

/// Key that unlocks the HTTP API when the access mode is `key`.
///
/// Redacted in `Debug` output.
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
