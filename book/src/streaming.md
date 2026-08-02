# Streaming

## screen

Frames come off the device as base64 text. The front screen is 72x16 BGR888. The backscreen is 80x80 8-bit grayscale
(despite that the screen on the device itself seems wider).

This is what "raw" means wrt the image format. Note that the API states the content-type is BMP, but this is not true.

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

An `--output` path ending in `.bmp`, `.jpg` or `.png` converts the frame to that image format. If the output path
extension is not one of these, you instead get raw bytes.

```console
busybar streaming screen front --output ./frame.png
```

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
