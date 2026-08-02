use busylib::model::smart_home::SwitchStartup;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum SwitchStartupArg {
    Off,
    On,
    Toggle,
    Last,
}

impl From<SwitchStartupArg> for SwitchStartup {
    fn from(startup: SwitchStartupArg) -> Self {
        match startup {
            SwitchStartupArg::Off => SwitchStartup::Off,
            SwitchStartupArg::On => SwitchStartup::On,
            SwitchStartupArg::Toggle => SwitchStartup::Toggle,
            SwitchStartupArg::Last => SwitchStartup::Last,
        }
    }
}
