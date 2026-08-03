# busy

## snapshot

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let snapshot = client.busy().snapshot().await?;

# Ok(())
# }
```

## set_snapshot

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();

let snapshot = client.busy().snapshot().await?;
client.busy().set_snapshot(&snapshot).await?;

# Ok(())
# }
```

## profile

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::busy::BusyProfileSlot;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let profile = client.busy().profile(BusyProfileSlot::Custom).await?;

# Ok(())
# }
```

## set_profile

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::busy::BusyProfileSlot;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();

let mut profile = client.busy().profile(BusyProfileSlot::Custom).await?;
profile.title = "study".to_owned();
client
    .busy()
    .set_profile(BusyProfileSlot::Custom, &profile)
    .await?;

# Ok(())
# }
```
