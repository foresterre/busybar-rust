crate::types::string_newtype!(
    /// Path to a file in an application's assets.
    AssetPath,
    "asset path",
    "one or more of [a-zA-Z0-9._/-]",
    crate::types::validate::asset_path
);
