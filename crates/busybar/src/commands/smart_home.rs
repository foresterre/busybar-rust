use busylib::model::smart_home::SmartHomeSwitchState;
use clap::Subcommand;

use crate::cli::Context;
use crate::error::Result;
use crate::reporter::{
    OkEvent, SmartHomePairingEvent, SmartHomeStartPairingEvent, SmartHomeSwitchEvent,
};
use crate::values::{SwitchStartupArg, SwitchStateArg};

#[derive(Debug, Subcommand)]
pub enum SmartHomeCommand {
    /// Show the smart home pairing status
    Pairing,

    /// Start pairing with a smart home
    StartPairing,

    /// Erase every smart home link
    ErasePairings,

    /// Show the state of the emulated switch
    Switch,

    /// Set the state of the emulated switch
    SetSwitch {
        /// Switch state
        #[arg(value_enum, value_name = "STATE")]
        state: SwitchStateArg,

        /// State of the switch after a restart
        #[arg(long, value_enum, value_name = "STATE")]
        startup: Option<SwitchStartupArg>,
    },
}

impl SmartHomeCommand {
    pub async fn run(self, context: &Context) -> Result<()> {
        match self {
            SmartHomeCommand::Pairing => pairing(context).await,
            SmartHomeCommand::StartPairing => start_pairing(context).await,
            SmartHomeCommand::ErasePairings => erase_pairings(context).await,
            SmartHomeCommand::Switch => switch(context).await,
            SmartHomeCommand::SetSwitch { state, startup } => {
                set_switch(context, state, startup).await
            }
        }
    }
}

async fn pairing(context: &Context) -> Result<()> {
    let pairing = context.client.smart_home().pairing().await?;

    context
        .reporter
        .report(SmartHomePairingEvent::new(pairing))?;

    Ok(())
}

async fn start_pairing(context: &Context) -> Result<()> {
    let payload = context.client.smart_home().start_pairing().await?;

    context
        .reporter
        .report(SmartHomeStartPairingEvent::new(payload))?;

    Ok(())
}

async fn erase_pairings(context: &Context) -> Result<()> {
    context.client.smart_home().erase_pairings().await?;

    context
        .reporter
        .report(OkEvent::new("smart-home erase-pairings"))?;

    Ok(())
}

async fn switch(context: &Context) -> Result<()> {
    let switch = context.client.smart_home().switch().await?;

    context.reporter.report(SmartHomeSwitchEvent::new(switch))?;

    Ok(())
}

async fn set_switch(
    context: &Context,
    state: SwitchStateArg,
    startup: Option<SwitchStartupArg>,
) -> Result<()> {
    let mut switch = if bool::from(state) {
        SmartHomeSwitchState::on()
    } else {
        SmartHomeSwitchState::off()
    };

    if let Some(startup) = startup {
        switch = switch.startup(startup.into());
    }

    context.client.smart_home().set_switch(&switch).await?;

    context
        .reporter
        .report(OkEvent::new("smart-home set-switch"))?;

    Ok(())
}
