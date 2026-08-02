//! Times of day

crate::types::string_newtype!(
    /// Time of day in `HH:MM`.
    TimeOfDay,
    "time of day",
    "a 24 hour time in HH:MM format",
    crate::types::validate::time_of_day
);
