# Busy

## snapshot

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let snapshot = client.busy().snapshot().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar busy snapshot
```

## set_snapshot

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();

let snapshot = client.busy().snapshot().await?;
client.busy().set_snapshot(&snapshot).await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar busy set-snapshot ./snapshot.json
```

## profile

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::busy::BusyProfileSlot;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let profile = client.busy().profile(BusyProfileSlot::Custom).await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar busy profile custom
```

## set_profile

<h3>API client</h3>

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

<h3>CLI</h3>

```console
busybar busy set-profile custom ./profile.json
```
