use crate::client::Call;
use crate::error::Result;
use crate::model::WifiStatus;
use crate::transport::HttpTransport;

crate::api::endpoint!(Wifi);

impl<T: HttpTransport> Wifi<'_, T> {
    pub async fn status(&self) -> Result<WifiStatus> {
        self.client.json(Call::get("/busybar/wifi/status")).await
    }
}
