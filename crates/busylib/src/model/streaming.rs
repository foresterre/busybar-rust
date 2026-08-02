use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamControl {
    pub enable: bool,
}

impl StreamControl {
    pub const fn enable() -> Self {
        Self { enable: true }
    }

    pub const fn disable() -> Self {
        Self { enable: false }
    }
}
