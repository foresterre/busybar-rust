use crate::client::Call;
use crate::error::Result;
use crate::model::smart_home::{
    SmartHomePairingInfo, SmartHomePairingPayload, SmartHomeSwitchState,
};
use crate::transport::HttpTransport;

crate::api::endpoint!(SmartHome);

impl<T: HttpTransport> SmartHome<'_, T> {
    pub async fn pairing(&self) -> Result<SmartHomePairingInfo> {
        self.client
            .json(Call::get("/busybar/smart_home/pairing"))
            .await
    }

    pub async fn start_pairing(&self) -> Result<SmartHomePairingPayload> {
        self.client
            .json(Call::post("/busybar/smart_home/pairing"))
            .await
    }

    pub async fn erase_pairings(&self) -> Result<()> {
        self.client
            .ok(Call::delete("/busybar/smart_home/pairing"))
            .await
    }

    pub async fn switch(&self) -> Result<SmartHomeSwitchState> {
        self.client
            .json(Call::get("/busybar/smart_home/switch"))
            .await
    }

    pub async fn set_switch(&self, switch: &SmartHomeSwitchState) -> Result<()> {
        let request = Call::post("/busybar/smart_home/switch").json(switch)?;
        self.client.ok(request).await
    }
}
