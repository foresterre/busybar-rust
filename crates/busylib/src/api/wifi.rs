use crate::client::Call;
use crate::error::Result;
use crate::model::wifi::StatusResponse;
use crate::transport::HttpTransport;

crate::api::endpoint!(Wifi);

impl<T: HttpTransport> Wifi<'_, T> {
    /// Returns current Wi-Fi status
    pub async fn status(&self) -> Result<StatusResponse> {
        self.client.json(Call::get("/busybar/wifi/status")).await
    }
}
