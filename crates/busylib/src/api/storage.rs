use bytes::Bytes;

use crate::client::Call;
use crate::error::Result;
use crate::model::storage::{StorageList, StorageListElement, StorageStatus};
use crate::transport::HttpTransport;
use crate::types::{StoragePath, TryIntoValue};

crate::api::endpoint!(Storage);

impl<T: HttpTransport> Storage<'_, T> {
    pub async fn write(
        &self,
        path: impl TryIntoValue<StoragePath>,
        data: impl Into<Bytes>,
    ) -> Result<()> {
        let request = Call::post("/busybar/storage/write")
            .query("path", path.try_into_value()?)
            .octet_stream(data);
        self.client.ok(request).await
    }

    pub async fn read(&self, path: impl TryIntoValue<StoragePath>) -> Result<Bytes> {
        let request = Call::get("/busybar/storage/read").query("path", path.try_into_value()?);
        self.client.bytes(request).await
    }

    pub async fn list(
        &self,
        path: impl TryIntoValue<StoragePath>,
    ) -> Result<Vec<StorageListElement>> {
        let request = Call::get("/busybar/storage/list").query("path", path.try_into_value()?);
        let response: StorageList = self.client.json(request).await?;
        Ok(response.list)
    }

    pub async fn remove(&self, path: impl TryIntoValue<StoragePath>) -> Result<()> {
        let request = Call::delete("/busybar/storage/remove").query("path", path.try_into_value()?);
        self.client.ok(request).await
    }

    pub async fn create_dir(&self, path: impl TryIntoValue<StoragePath>) -> Result<()> {
        let request = Call::post("/busybar/storage/mkdir").query("path", path.try_into_value()?);
        self.client.ok(request).await
    }

    pub async fn rename(
        &self,
        path: impl TryIntoValue<StoragePath>,
        new_path: impl TryIntoValue<StoragePath>,
    ) -> Result<()> {
        let request = Call::post("/busybar/storage/rename")
            .query("path", path.try_into_value()?)
            .query("new_path", new_path.try_into_value()?);
        self.client.ok(request).await
    }

    pub async fn status(&self) -> Result<StorageStatus> {
        self.client.json(Call::get("/busybar/storage/status")).await
    }
}
