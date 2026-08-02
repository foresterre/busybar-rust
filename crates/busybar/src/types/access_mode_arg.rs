use busylib::model::settings::AccessMode;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum AccessModeArg {
    Disabled,
    Enabled,
    Key,
}

impl From<AccessModeArg> for AccessMode {
    fn from(mode: AccessModeArg) -> Self {
        match mode {
            AccessModeArg::Disabled => AccessMode::Disabled,
            AccessModeArg::Enabled => AccessMode::Enabled,
            AccessModeArg::Key => AccessMode::Key,
        }
    }
}
