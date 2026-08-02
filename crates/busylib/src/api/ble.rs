use crate::client::Call;
use crate::error::Result;
use crate::model::ble::BleStatusResponse;
use crate::transport::HttpTransport;

crate::api::endpoint!(Ble);

impl<T: HttpTransport> Ble<'_, T> {
    pub async fn enable(&self) -> Result<()> {
        self.client.ok(Call::post("/busybar/ble/enable")).await
    }

    pub async fn disable(&self) -> Result<()> {
        self.client.ok(Call::post("/busybar/ble/disable")).await
    }

    pub async fn remove_pairing(&self) -> Result<()> {
        self.client.ok(Call::delete("/busybar/ble/pairing")).await
    }

    pub async fn status(&self) -> Result<BleStatusResponse> {
        self.client.json(Call::get("/busybar/ble/status")).await
    }
}
