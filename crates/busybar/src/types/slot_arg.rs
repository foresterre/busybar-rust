use busylib::model::busy::BusyProfileSlot;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum SlotArg {
    Busy,
    Custom,
}

impl From<SlotArg> for BusyProfileSlot {
    fn from(slot: SlotArg) -> Self {
        match slot {
            SlotArg::Busy => BusyProfileSlot::Busy,
            SlotArg::Custom => BusyProfileSlot::Custom,
        }
    }
}
