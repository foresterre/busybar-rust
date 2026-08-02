//! Asset endpoints

use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::assets::{DisplayElements, PlayAudio};
use crate::transport::HttpTransport;
use crate::types::app_name::AppName;
use crate::types::asset_name::AssetName;
use crate::types::try_into_value::TryIntoValue;

crate::api::endpoint!(
    /// Asset files, and the audio and display calls that play them back
    Assets
);

impl<T: HttpTransport> Assets<'_, T> {
    /// Upload asset file with app ID
    ///
    /// Uploads a file to a specific app's assets directory
    pub async fn upload(
        &self,
        application_name: impl TryIntoValue<AppName>,
        file: impl TryIntoValue<AssetName>,
        data: impl Into<Bytes>,
    ) -> Result<()> {
        let request = Call::post("assets/upload")
            .query("application_name", application_name.try_into_value()?)
            .query("file", file.try_into_value()?)
            .octet_stream(data);
        self.client.ok(request).await
    }

    /// Delete app assets
    ///
    /// Deletes all assets for a specific app ID
    pub async fn delete(&self, application_name: impl TryIntoValue<AppName>) -> Result<()> {
        let request = Call::delete("assets/upload")
            .query("application_name", application_name.try_into_value()?);
        self.client.ok(request).await
    }

    /// Play audio file
    ///
    /// Plays an audio file from the assets directory.
    /// Supported formats include .snd files.
    pub async fn play(&self, audio: &PlayAudio) -> Result<()> {
        let request = Call::post("audio/play").json(audio)?;
        self.client.ok(request).await
    }

    /// Stop audio playback
    ///
    /// Stops any currently playing audio
    pub async fn stop(&self) -> Result<()> {
        self.client.ok(Call::delete("audio/play")).await
    }

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
    pub async fn clear(&self, application_name: Option<AppName>) -> Result<()> {
        let request =
            Call::delete("display/draw").maybe_query("application_name", application_name);
        self.client.ok(request).await
    }
}
