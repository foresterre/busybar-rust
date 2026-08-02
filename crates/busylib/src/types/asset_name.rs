crate::types::string_newtype!(
    /// File name of an asset within an application.
    AssetName,
    "asset file name",
    "one or more of [a-zA-Z0-9._-]",
    crate::types::validate::name
);
