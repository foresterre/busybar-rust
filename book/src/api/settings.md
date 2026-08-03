# settings

## access

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let access = client.settings().access().await?;

# Ok(())
# }
```

## set_access

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::settings::HttpAccess;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client
    .settings()
    .set_access(&HttpAccess::Key("12345678".parse()?))
    .await?;

# Ok(())
# }
```

## name

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let name = client.settings().name().await?;

# Ok(())
# }
```

## set_name

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.settings().set_name("BUSY bar").await?;

# Ok(())
# }
```

## volume

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let volume = client.settings().volume().await?;

# Ok(())
# }
```

## set_volume

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::types::volume::Volume;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.settings().set_volume(Volume::new(35)?, true).await?;

# Ok(())
# }
```

## brightness

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let brightness = client.settings().brightness().await?;

# Ok(())
# }
```

## set_brightness

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::types::brightness::Brightness;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client
    .settings()
    .set_brightness(Brightness::level(40)?)
    .await?;

# Ok(())
# }
```
