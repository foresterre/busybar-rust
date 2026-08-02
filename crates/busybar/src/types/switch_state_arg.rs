use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum SwitchStateArg {
    On,
    Off,
}

impl From<SwitchStateArg> for bool {
    fn from(state: SwitchStateArg) -> Self {
        matches!(state, SwitchStateArg::On)
    }
}
