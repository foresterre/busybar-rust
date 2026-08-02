use crate::reporter;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error(transparent)]
    Api(#[from] busylib::Error),

    #[error("could not render the result as JSON")]
    Json(#[from] serde_json::Error),

    #[error("could not read or write a local file")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Usage(&'static str),

    #[error(transparent)]
    Reporter(#[from] reporter::ReporterError),
}

impl From<busylib::types::invalid_value::InvalidValue> for CliError {
    fn from(error: busylib::types::invalid_value::InvalidValue) -> Self {
        Self::Api(busylib::Error::Value(error))
    }
}
