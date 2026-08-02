crate::types::string_newtype!(
    /// Application name that groups assets and drawn elements.
    AppName,
    "application name",
    "one or more of [a-zA-Z0-9._-]",
    crate::types::validate::name
);
