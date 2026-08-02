//! Smart home endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::smart_home::{
    SmartHomePairingInfo, SmartHomePairingPayload, SmartHomeSwitchState,
};
use crate::transport::HttpTransport;

crate::api::endpoint!(
    /// Smart Home event handling
    SmartHome
);

impl<T: HttpTransport> SmartHome<'_, T> {
    /// Smart home commissioning status
    pub async fn pairing(&self) -> Result<SmartHomePairingInfo> {
        self.client.json(Call::get("smart_home/pairing")).await
    }

    /// Link device to a smart home
    pub async fn start_pairing(&self) -> Result<SmartHomePairingPayload> {
        self.client.json(Call::post("smart_home/pairing")).await
    }

    /// Erase all smart home links
    pub async fn erase_pairings(&self) -> Result<()> {
        self.client.ok(Call::delete("smart_home/pairing")).await
    }

    /// Get state of emulated smart home switch
    pub async fn switch(&self) -> Result<SmartHomeSwitchState> {
        self.client.json(Call::get("smart_home/switch")).await
    }

    /// Set state of emulated smart home switch
    pub async fn set_switch(&self, switch: &SmartHomeSwitchState) -> Result<()> {
        let request = Call::post("smart_home/switch").json(switch)?;
        self.client.ok(request).await
    }
}
