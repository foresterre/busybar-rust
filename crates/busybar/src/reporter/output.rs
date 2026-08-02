use std::io::{self, Write};
use std::sync::{Mutex, PoisonError};

#[derive(Debug, thiserror::Error)]
pub enum OutputError {
    #[error("could not write to the output")]
    Io(#[from] io::Error),

    #[error("could not render the event as JSON")]
    Json(#[from] serde_json::Error),
}

pub struct Output {
    writer: Mutex<Box<dyn Write + Send>>,
    error: Mutex<Option<OutputError>>,
}

impl Output {
    pub fn stdout() -> Self {
        Self::new(io::stdout())
    }

    pub fn new(writer: impl Write + Send + 'static) -> Self {
        Self {
            writer: Mutex::new(Box::new(writer)),
            error: Mutex::new(None),
        }
    }

    pub fn write_line(&self, line: &str) {
        let mut writer = self.writer.lock().unwrap_or_else(PoisonError::into_inner);

        if let Err(error) = writeln!(writer, "{line}") {
            drop(writer);
            self.record_error(error.into());
        }
    }

    pub fn flush(&self) {
        let mut writer = self.writer.lock().unwrap_or_else(PoisonError::into_inner);

        if let Err(error) = writer.flush() {
            drop(writer);
            self.record_error(error.into());
        }
    }

    pub fn record_error(&self, error: OutputError) {
        let mut slot = self.error.lock().unwrap_or_else(PoisonError::into_inner);

        if slot.is_none() {
            *slot = Some(error);
        }
    }

    pub fn take_error(&self) -> Option<OutputError> {
        self.error
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }
}

impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Output")
    }
}
