use crate::reporter;

pub type Result<T> = std::result::Result<T, CliError>;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error(transparent)]
    Api(#[from] busylib::Error),

    #[error("could not render the result as JSON")]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Reporter(#[from] reporter::ReporterError),
}

impl From<busylib::InvalidValue> for CliError {
    fn from(error: busylib::InvalidValue) -> Self {
        Self::Api(busylib::Error::Value(error))
    }
}
