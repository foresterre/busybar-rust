//! Conversion into validated values

use crate::types::invalid_value::InvalidValue;

/// Conversion into a validated value, accepting raw and already validated inputs.
pub trait TryIntoValue<T> {
    fn try_into_value(self) -> Result<T, InvalidValue>;
}

impl<T> TryIntoValue<T> for T {
    fn try_into_value(self) -> Result<T, InvalidValue> {
        Ok(self)
    }
}
