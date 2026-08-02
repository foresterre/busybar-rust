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

<h3>API client</h3>

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

<h3>CLI</h3>

Prints one numbered line per message as it arrives, until interrupted with ctrl-c. With
`--output-format json` each line is a JSON object (aka jsonlines), so the stream can for example be piped into `jq`.

```console
busybar streaming status-ws
```

If you provide a `--frame-dir <dir>` option, `busybar` as a side effect, will decode the streamed frames and write
them as images to the given folder.

By default, front frames are rendered with a black raster to mimic the
matrix display of the actual device. This can be disabled by providing the `--no-image-raster` flag.

```console
busybar streaming status-ws --frame-dir ./frames
```

Reported events carry the frame inline as base64, in the same `--image-format` the files use.

```console
busybar --image-format jpg -o json streaming status-ws
```
