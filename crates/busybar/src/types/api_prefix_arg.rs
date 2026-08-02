use busylib::ApiPrefix;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ApiPrefixArg {
    Device,
    Cloud,
}

impl From<ApiPrefixArg> for ApiPrefix {
    fn from(prefix: ApiPrefixArg) -> Self {
        match prefix {
            ApiPrefixArg::Device => ApiPrefix::Device,
            ApiPrefixArg::Cloud => ApiPrefix::Cloud,
        }
    }
}
