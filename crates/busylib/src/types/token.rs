//! API tokens

use std::fmt;
use std::str::FromStr;

use crate::types::invalid_value::InvalidValue;
use crate::types::try_into_value::TryIntoValue;

/// BUSY Cloud BAR-scope API token.
///
/// Redacted in `Debug` output.
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
