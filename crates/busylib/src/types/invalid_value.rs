/// Reason a value was rejected by a validating constructor.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("invalid {label} `{value}`: expected {expectation}")]
pub struct InvalidValue {
    label: &'static str,
    value: String,
    expectation: &'static str,
}

impl InvalidValue {
    pub(crate) fn new(
        label: &'static str,
        value: impl Into<String>,
        expectation: &'static str,
    ) -> Self {
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
