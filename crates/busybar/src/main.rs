mod cli;
mod commands;
mod error;
mod io;
mod reporter;
pub mod types;

use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::error::CliError;

#[tokio::main]
async fn main() -> ExitCode {
    match Cli::parse().run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            report(&error);
            ExitCode::FAILURE
        }
    }
}

fn report(error: &CliError) {
    eprintln!("error: {error}");
    let mut source = std::error::Error::source(error);
    while let Some(cause) = source {
        eprintln!("  caused by: {cause}");
        source = cause.source();
    }
}
