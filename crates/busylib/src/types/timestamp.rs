//! Timestamps

crate::types::string_newtype!(
    /// ISO 8601 timestamp with a time zone.
    Timestamp,
    "timestamp",
    "an ISO 8601 timestamp with time zone, such as 2025-10-02T14:30:45+02:00",
    crate::types::validate::timestamp
);
