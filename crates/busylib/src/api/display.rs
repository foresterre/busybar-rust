//! Display endpoints

use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::display::{DisplayBrightnessInfo, DisplayElements, Screen};
use crate::transport::HttpTransport;
use crate::types::app_name::AppName;
use crate::types::brightness::Brightness;
use crate::types::try_into_value::TryIntoValue;

crate::api::endpoint!(
    /// Display control
    Display
);

impl<T: HttpTransport> Display<'_, T> {
    /// Draw on display
    ///
    /// Sends drawing data to the display.
    /// Supports JSON-defined display elements.
    pub async fn draw(&self, elements: &DisplayElements) -> Result<()> {
        let request = Call::post("display/draw").json(elements)?;
        self.client.ok(request).await
    }

    /// Clear display
    ///
    /// Deletes display elements drawn by the Canvas application. If application_name is specified,
    /// only elements for that app are removed.
    pub async fn clear(&self) -> Result<()> {
        self.client.ok(Call::delete("display/draw")).await
    }

    /// Clear display
    ///
    /// Deletes display elements drawn by the Canvas application. If application_name is specified,
    /// only elements for that app are removed.
    pub async fn clear_app(&self, application_name: impl TryIntoValue<AppName>) -> Result<()> {
        let request = Call::delete("display/draw")
            .query("application_name", application_name.try_into_value()?);
        self.client.ok(request).await
    }

    /// Get display brightness
    ///
    /// Get brightness value for displays
    pub async fn brightness(&self) -> Result<Brightness> {
        let response: DisplayBrightnessInfo =
            self.client.json(Call::get("display/brightness")).await?;
        Ok(response.value)
    }

    /// Set display brightness
    ///
    /// Set brightness for one or both displays
    pub async fn set_brightness(&self, brightness: Brightness) -> Result<()> {
        let request = Call::post("display/brightness").query("value", brightness);
        self.client.ok(request).await
    }

    /// Get single frame for requested screen
    pub async fn frame(&self, screen: Screen) -> Result<Bytes> {
        let request = Call::get("screen").query("display", screen.index());
        self.client.bytes(request).await
    }
}
