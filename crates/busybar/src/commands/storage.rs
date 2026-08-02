use std::path::PathBuf;

use busylib::StoragePath;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;

#[derive(Debug, Subcommand)]
pub enum StorageCommand {
    /// Upload a file to the internal storage
    Write {
        /// Destination below /ext
        #[arg(value_name = "PATH")]
        path: StoragePath,

        /// Local file to upload, or - for stdin
        #[arg(long, short = 'f', value_name = "FILE")]
        file: PathBuf,
    },

    /// Download a file from the internal storage
    Read {
        /// File to download
        #[arg(value_name = "PATH")]
        path: StoragePath,

        /// Write the file here instead of stdout
        #[arg(long, short = 'O', value_name = "FILE")]
        output: Option<PathBuf>,
    },

    /// List a directory on the internal storage
    List {
        /// Directory to list
        #[arg(value_name = "PATH", default_value = "/ext")]
        path: StoragePath,
    },

    /// Remove a file from the internal storage
    Remove {
        /// File to remove
        #[arg(value_name = "PATH")]
        path: StoragePath,
    },

    /// Create a directory on the internal storage
    Mkdir {
        /// Directory to create
        #[arg(value_name = "PATH")]
        path: StoragePath,
    },

    /// Move a file on the internal storage
    Rename {
        /// Current location
        #[arg(value_name = "PATH")]
        path: StoragePath,

        /// New location
        #[arg(value_name = "NEW_PATH")]
        new_path: StoragePath,
    },

    /// Show the storage usage
    Status,
}

impl StorageCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
