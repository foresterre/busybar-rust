use std::fmt::Formatter;
use std::str::FromStr;
use std::{fmt, time};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeoutArg(time::Duration);

impl TimeoutArg {
    pub const DEFAULT: Self = Self(time::Duration::from_secs(10));

    pub fn to_duration(&self) -> time::Duration {
        self.0
    }
}

impl FromStr for TimeoutArg {
    type Err = TimeoutArgError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let ms: u64 = input.parse().map_err(|_| TimeoutArgError::Duration {
            input: input.to_string(),
        })?;

        Ok(Self(time::Duration::from_millis(ms)))
    }
}

impl fmt::Display for TimeoutArg {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.0.as_millis()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TimeoutArgError {
    #[error("the {input} couldn't be parsed as a millisecond duration")]
    Duration { input: String },
}
