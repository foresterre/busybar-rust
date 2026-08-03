# Settings

## access

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let access = client.settings().access().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api settings access
```

## set_access

<h3>API client</h3>

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

<h3>CLI</h3>

```console
busybar api settings set-access key --key 12345678
```

## name

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let name = client.settings().name().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api settings name
```

## set_name

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.settings().set_name("BUSY bar").await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api settings set-name "BUSY bar"
```

## volume

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let volume = client.settings().volume().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api settings volume
```

## set_volume

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::types::volume::Volume;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.settings().set_volume(Volume::new(35)?, true).await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api settings set-volume 35 --silent
```

## brightness

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let brightness = client.settings().brightness().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api settings brightness
```

## set_brightness

<h3>API client</h3>

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

<h3>CLI</h3>

```console
busybar api settings set-brightness 40
```
