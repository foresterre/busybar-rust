# Streaming

## screen

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::assets::Screen;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let frame = client.streaming().screen(Screen::Front).await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar streaming screen front --output ./frame.raw
```

## status_ws

⚠️ *not implemented yet*

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.streaming().status_ws().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar streaming status-ws
```
