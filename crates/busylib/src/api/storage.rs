//! Storage endpoints

use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::storage::{StorageList, StorageListElement, StorageStatus};
use crate::transport::HttpTransport;
use crate::types::storage_path::StoragePath;
use crate::types::try_into_value::TryIntoValue;

crate::api::endpoint!(
    /// File storage operations
    Storage
);

impl<T: HttpTransport> Storage<'_, T> {
    /// Upload file to internal storage
    ///
    /// Uploads a file to a specified path
    pub async fn write(
        &self,
        path: impl TryIntoValue<StoragePath>,
        data: impl Into<Bytes>,
    ) -> Result<()> {
        let request = Call::post("storage/write")
            .query("path", path.try_into_value()?)
            .octet_stream(data);
        self.client.ok(request).await
    }

    /// Download file from internal storage
    ///
    /// Downloads a file from a specified path
    pub async fn read(&self, path: impl TryIntoValue<StoragePath>) -> Result<Bytes> {
        let request = Call::get("storage/read").query("path", path.try_into_value()?);
        self.client.bytes(request).await
    }

    /// List files on internal storage
    pub async fn list(
        &self,
        path: impl TryIntoValue<StoragePath>,
    ) -> Result<Vec<StorageListElement>> {
        let request = Call::get("storage/list").query("path", path.try_into_value()?);
        let response: StorageList = self.client.json(request).await?;
        Ok(response.list)
    }

    /// Remove a file on internal storage
    ///
    /// Removes a file with a specified path
    pub async fn remove(&self, path: impl TryIntoValue<StoragePath>) -> Result<()> {
        let request = Call::delete("storage/remove").query("path", path.try_into_value()?);
        self.client.ok(request).await
    }

    /// Create a directory on internal storage
    ///
    /// Creates a new directory with a specified path
    pub async fn mkdir(&self, path: impl TryIntoValue<StoragePath>) -> Result<()> {
        let request = Call::post("storage/mkdir").query("path", path.try_into_value()?);
        self.client.ok(request).await
    }

    /// Rename/move a file
    ///
    /// Moves a file to a new location
    pub async fn rename(
        &self,
        path: impl TryIntoValue<StoragePath>,
        new_path: impl TryIntoValue<StoragePath>,
    ) -> Result<()> {
        let request = Call::post("storage/rename")
            .query("path", path.try_into_value()?)
            .query("new_path", new_path.try_into_value()?);
        self.client.ok(request).await
    }

    /// Show storage usage
    pub async fn status(&self) -> Result<StorageStatus> {
        self.client.json(Call::get("storage/status")).await
    }
}
