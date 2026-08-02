use busylib::{Client, ClientBuilder, ReqwestHttpTransport};
use clap::Parser;

use crate::commands::Command;
use crate::error::Result;
use crate::values::OutputFormat;

/// Control a BUSY Bar over its HTTP API, using this CLI <3
#[derive(Debug, Parser)]
#[command(name = "busybar", version, propagate_version = true)]
pub struct Cli {
    /// Base URL of the device
    #[arg(
        long,
        env = "BUSYBAR_URL",
        value_name = "URL",
        default_value = "http://10.0.4.20"
    )]
    url: String,

    /// BUSY Cloud BAR-scope API token
    #[arg(
        long,
        env = "BUSYBAR_TOKEN",
        value_name = "TOKEN",
        hide_env_values = true
    )]
    token: Option<String>,

    /// The output format, i.e. how to print thje result
    #[arg(long, short = 'o', value_enum, default_value_t = OutputFormat::Text)]
    output_format: OutputFormat,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let Cli {
            url,
            token,
            output_format,
            command,
        } = self;

        let mut builder = ClientBuilder::new(&url)?;
        if let Some(token) = token {
            builder = builder.token(token)?;
        }

        let context = Context {
            client: builder.build_reqwest(),
            output_format,
        };
        command.run(&context).await
    }
}

#[derive(Debug)]
pub struct Context {
    #[allow(unused)] // TODO Use when implemented
    pub client: Client<ReqwestHttpTransport>,
    #[allow(unused)] // TODO add storyteller and impl
    pub output_format: OutputFormat,
}
