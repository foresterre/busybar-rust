//! BLE endpoints

use crate::client::Call;
use crate::error::Result;
use crate::model::ble::BleStatusResponse;
use crate::transport::HttpTransport;

crate::api::endpoint!(
    /// Allows to operate with BLE
    Ble
);

impl<T: HttpTransport> Ble<'_, T> {
    /// Enable BLE
    ///
    /// Enables BLE module and starts advertising
    pub async fn enable(&self) -> Result<()> {
        self.client.ok(Call::post("ble/enable")).await
    }

    /// Disable BLE
    ///
    /// Stops advertising
    pub async fn disable(&self) -> Result<()> {
        self.client.ok(Call::post("ble/disable")).await
    }

    /// Remove pairing
    ///
    /// Remove pairing with previous device
    pub async fn remove_pairing(&self) -> Result<()> {
        self.client.ok(Call::delete("ble/pairing")).await
    }

    /// Returns current BLE status
    pub async fn status(&self) -> Result<BleStatusResponse> {
        self.client.json(Call::get("ble/status")).await
    }
}
