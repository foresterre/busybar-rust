use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::OkEvent;
use crate::types::key_arg::KeyArg;

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
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            InputCommand::Press { key } => press(context, key).await,
        }
    }
}

async fn press(context: &Context, key: KeyArg) -> Result<()> {
    context.client.input().press(key.into()).await?;

    context.reporter.report(OkEvent::new("input press"))?;

    Ok(())
}
