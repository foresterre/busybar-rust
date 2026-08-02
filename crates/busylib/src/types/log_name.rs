//! Log file names

crate::types::string_newtype!(
    /// Name of a log dump file, without extension.
    LogName,
    "log file name",
    "one or more of [a-zA-Z0-9_-], without extension",
    crate::types::validate::log_name
);
