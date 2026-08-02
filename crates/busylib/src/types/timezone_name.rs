crate::types::string_newtype!(
    /// Name of a time zone the device accepts.
    TimezoneName,
    "time zone name",
    "a letter followed by up to 50 of [A-Za-z0-9 _+-]",
    crate::types::validate::timezone_name
);
