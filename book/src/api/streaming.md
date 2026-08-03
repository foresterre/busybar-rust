# streaming

## screen

Frames come off the device as base64 text. The front screen is 72x16 BGR888. The backscreen is 80x80 8-bit grayscale
(despite that the screen on the device itself seems wider).

This is what "raw" means wrt the image format. 

Note that the API states the content-type is BMP, but this is not true. It may also say its RGB888, but this is also not
true (it is BGR888 instead, so you should swap the blue and red channels if you want RGB). 

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::assets::Screen;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let frame = client.streaming().screen(Screen::Front).await?;

# Ok(())
# }
```

## status_ws

The stream is enabled as soon as it opens, and ends when the device hangs up.
The WebSocket transport is passed per call, since the client itself currently only carries an HTTP transport.

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::{ClientBuilder, TungsteniteWsTransport};

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let mut stream = client
    .streaming()
    .status_ws(&TungsteniteWsTransport::new())
    .await?;

while let Some(message) = stream.next().await {
    let message = message?;
}

# Ok(())
# }
```
