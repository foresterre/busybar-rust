use busylib::{Client, ClientBuilder, ReqwestHttpTransport};
use clap::Parser;

use crate::commands::Command;
use crate::error::Result;
use crate::reporter::Reporter;
use crate::types::api_prefix_arg::ApiPrefixArg;
use crate::types::image_format_arg::ImageFormatArg;
use crate::types::output_format::OutputFormatArg;

/// Control a BUSY Bar over its HTTP API, using this CLI <3
#[derive(Debug, Parser)]
#[command(name = "busybar", version)]
pub struct Cli {
    /// Base URL of the device
    #[arg(
        long,
        env = "BUSYBAR_URL",
        value_name = "URL",
        default_value = "http://10.0.4.20"
    )]
    url: String,

    /// Path the API is mounted under: `/api` on a bar, `/busybar` on OpenAPI spec
    #[arg(long, value_enum, default_value_t = ApiPrefixArg::Device)]
    api_prefix: ApiPrefixArg,

    /// BUSY Cloud BAR-scope API token
    #[arg(
        long,
        env = "BUSYBAR_TOKEN",
        value_name = "TOKEN",
        hide_env_values = true
    )]
    token: Option<String>,

    /// The output format, i.e. how to print thje result
    #[arg(long, short = 'o', value_enum, default_value_t = OutputFormatArg::Text)]
    output_format: OutputFormatArg,

    /// Format that frames are written and reported in, `raw` passes the device bytes through
    #[arg(long, value_enum, default_value_t = ImageFormatArg::Png)]
    image_format: ImageFormatArg,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let Cli {
            url,
            api_prefix,
            token,
            output_format,
            image_format,
            command,
        } = self;

        let mut builder = ClientBuilder::new(&url)?.api_prefix(api_prefix.into());
        if let Some(token) = token {
            builder = builder.token(token)?;
        }

        let context = Context {
            client: builder.build_reqwest(),
            reporter: Reporter::new(output_format),
            output_format,
            image_format,
        };

        let result = command.run(&context).await;
        let Context { reporter, .. } = context;

        result.and(reporter.finish().map_err(Into::into))
    }
}

#[derive(Debug)]
pub struct Context {
    pub client: Client<ReqwestHttpTransport>,
    pub reporter: Reporter,
    pub output_format: OutputFormatArg,
    pub image_format: ImageFormatArg,
}
