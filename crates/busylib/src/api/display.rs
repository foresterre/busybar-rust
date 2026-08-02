use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::display::{DisplayBrightnessInfo, DisplayElements, Screen};
use crate::transport::HttpTransport;
use crate::types::{AppName, Brightness, TryIntoValue};

crate::api::endpoint!(Display);

impl<T: HttpTransport> Display<'_, T> {
    pub async fn draw(&self, elements: &DisplayElements) -> Result<()> {
        let request = Call::post("/busybar/display/draw").json(elements)?;
        self.client.ok(request).await
    }

    pub async fn clear(&self) -> Result<()> {
        self.client.ok(Call::delete("/busybar/display/draw")).await
    }

    pub async fn clear_app(&self, application_name: impl TryIntoValue<AppName>) -> Result<()> {
        let request = Call::delete("/busybar/display/draw")
            .query("application_name", application_name.try_into_value()?);
        self.client.ok(request).await
    }

    pub async fn brightness(&self) -> Result<Brightness> {
        let response: DisplayBrightnessInfo = self
            .client
            .json(Call::get("/busybar/display/brightness"))
            .await?;
        Ok(response.value)
    }

    pub async fn set_brightness(&self, brightness: Brightness) -> Result<()> {
        let request = Call::post("/busybar/display/brightness").query("value", brightness);
        self.client.ok(request).await
    }

    pub async fn frame(&self, screen: Screen) -> Result<Bytes> {
        let request = Call::get("/busybar/screen").query("display", screen.index());
        self.client.bytes(request).await
    }
}
