crate::types::string_newtype!(
    /// Identifier of a drawn element within an application.
    ElementId,
    "element id",
    "one or more of [a-zA-Z0-9._-]",
    crate::types::validate::name
);
