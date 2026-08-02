//! Request timeouts

use http::Request;
use std::fmt;
use std::time::Duration;

/// How long a request may take, until it times out
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timeout(Duration);

impl Timeout {
    pub const fn new(duration: Duration) -> Self {
        Self(duration)
    }

    pub const fn duration(self) -> Duration {
        self.0
    }

    pub fn of<B>(request: &Request<B>) -> Option<Duration> {
        request
            .extensions()
            .get::<Self>()
            .map(|timeout| timeout.duration())
    }
}

impl From<Duration> for Timeout {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

impl fmt::Display for Timeout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
