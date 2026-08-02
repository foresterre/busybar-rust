use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Payload {
    bytes: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base64: Option<String>,
}

impl Payload {
    pub fn written(data: &[u8], path: String) -> Self {
        Self {
            bytes: data.len(),
            path: Some(path),
            base64: None,
        }
    }

    pub fn inline(data: &[u8]) -> Self {
        Self {
            bytes: data.len(),
            path: None,
            base64: Some(STANDARD.encode(data)),
        }
    }
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.path {
            Some(path) => write!(f, "wrote {} bytes to {path}", self.bytes),
            None => write!(f, "{} bytes", self.bytes),
        }
    }
}
