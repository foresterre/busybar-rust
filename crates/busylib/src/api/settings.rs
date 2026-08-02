use crate::client::Call;
use crate::error::Result;
use crate::model::{HttpAccess, HttpAccessInfo, NameInfo};
use crate::transport::HttpTransport;
use crate::types::{DeviceName, TryIntoValue};

crate::api::endpoint!(Settings);

impl<T: HttpTransport> Settings<'_, T> {
    pub async fn http_access(&self) -> Result<HttpAccessInfo> {
        self.client.json(Call::get("/busybar/access")).await
    }

    pub async fn set_http_access(&self, access: &HttpAccess) -> Result<()> {
        let request = Call::post("/busybar/access")
            .query("mode", access.mode().as_str())
            .maybe_query("key", access.key().map(|key| key.as_str()));
        self.client.ok(request).await
    }

    pub async fn name(&self) -> Result<DeviceName> {
        let response: NameInfo = self.client.json(Call::get("/busybar/name")).await?;
        Ok(response.name)
    }

    pub async fn set_name(&self, name: impl TryIntoValue<DeviceName>) -> Result<()> {
        let body = NameInfo {
            name: name.try_into_value()?,
        };
        let request = Call::post("/busybar/name").json(&body)?;
        self.client.ok(request).await
    }
}
