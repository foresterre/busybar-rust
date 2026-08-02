use crate::client::Call;
use crate::error::Result;
use crate::model::ble::BleStatusResponse;
use crate::transport::HttpTransport;

crate::api::endpoint!(Ble);

impl<T: HttpTransport> Ble<'_, T> {
    /// Enable BLE
    ///
    /// Enables BLE module and starts advertising
    pub async fn enable(&self) -> Result<()> {
        self.client.ok(Call::post("/busybar/ble/enable")).await
    }

    /// Disable BLE
    ///
    /// Stops advertising
    pub async fn disable(&self) -> Result<()> {
        self.client.ok(Call::post("/busybar/ble/disable")).await
    }

    /// Remove pairing
    ///
    /// Remove pairing with previous device
    pub async fn remove_pairing(&self) -> Result<()> {
        self.client.ok(Call::delete("/busybar/ble/pairing")).await
    }

    /// Returns current BLE status
    pub async fn status(&self) -> Result<BleStatusResponse> {
        self.client.json(Call::get("/busybar/ble/status")).await
    }
}
