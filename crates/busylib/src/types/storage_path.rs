crate::types::string_newtype!(
    /// Path to a file or directory below `/ext`.
    StoragePath,
    "storage path",
    "`/ext` optionally followed by `/` separated segments of [a-zA-Z0-9._-]",
    crate::types::validate::storage_path
);
