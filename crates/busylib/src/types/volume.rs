//! Audio volume values

crate::types::percentage_newtype!(
    /// Audio volume, as a percentage.
    Volume,
    "volume",
    "a percentage between 0 and 100",
    0
);
