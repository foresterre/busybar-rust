use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::Result;
use crate::reporter::Payload;
use crate::types::output_format::OutputFormatArg;

const STDIO: &str = "-";

pub struct Io;

impl Io {
    pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
        if path == Path::new(STDIO) {
            let mut buffer = Vec::new();
            std::io::stdin().read_to_end(&mut buffer)?;
            return Ok(buffer);
        }

        Ok(fs::read(path)?)
    }

    pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
        let bytes = Self::read_bytes(path)?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub fn create_dir(path: &Path) -> Result<()> {
        Ok(fs::create_dir_all(path)?)
    }

    pub fn write_bytes(path: &Path, data: &[u8]) -> Result<()> {
        Ok(fs::write(path, data)?)
    }

    pub fn write_stdout(data: &[u8]) -> Result<()> {
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(data)?;
        stdout.flush()?;
        Ok(())
    }

    /// Hands binary data to the user: to a file when one is given, otherwise piped to stdout
    /// as raw bytes. If the output-formatt is json, it is instead encoded as base64
    /// and printed as a parameter of an event.
    ///
    /// Returns the payload to report for events, or `None` when the bytes went to stdout.
    pub fn output_binary_data(
        format: OutputFormatArg,
        data: &[u8],
        output: Option<&Path>,
    ) -> Result<Option<Payload>> {
        match output {
            Some(path) => {
                Self::write_bytes(path, data)?;
                Ok(Some(Payload::written(data, path.display().to_string())))
            }
            None => match format {
                OutputFormatArg::Json => Ok(Some(Payload::inline(data))),
                OutputFormatArg::Text => {
                    Self::write_stdout(data)?;
                    Ok(None)
                }
            },
        }
    }
}
