use crate::client::Call;
use crate::error::Result;
use crate::model::Key;
use crate::transport::HttpTransport;

crate::api::endpoint!(Input);

impl<T: HttpTransport> Input<'_, T> {
    pub async fn press(&self, key: Key) -> Result<()> {
        let request = Call::post("/busybar/input").query("key", key.as_str());
        self.client.ok(request).await
    }
}
