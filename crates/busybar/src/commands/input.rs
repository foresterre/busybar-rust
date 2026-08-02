use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::values::KeyArg;

#[derive(Debug, Subcommand)]
pub enum InputCommand {
    /// Send a single key press
    Press {
        /// Key to press
        #[arg(value_enum, value_name = "KEY")]
        key: KeyArg,
    },
}

impl InputCommand {
    pub async fn run(self, _context: &Context) -> Result<()> {
        todo!()
    }
}
