//! Display text

crate::types::string_newtype!(
    /// Line of text to draw.
    Text,
    "display text",
    "one or more printable ASCII characters",
    crate::types::validate::printable_ascii
);
