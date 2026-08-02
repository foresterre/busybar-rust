//! Device names

crate::types::string_newtype!(
    /// Name the device is known by.
    DeviceName,
    "device name",
    "1 to 20 letters, digits, spaces or common punctuation (no backtick or tilde)",
    crate::types::validate::device_name
);
