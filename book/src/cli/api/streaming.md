# streaming

## screen

An `--output` path ending in `.bmp`, `.jpg` or `.png` converts the frame to that image format. If the output path
extension is not one of these, you instead get raw bytes.

```console
busybar api streaming screen front --output ./frame.png
```

```console
busybar api streaming screen front --output ./frame.raw
```

## status_ws

Prints one numbered line per message as it arrives, until interrupted with ctrl-c. With
`--output-format json` each line is a JSON object (aka jsonlines), so the stream can for example be piped into `jq`.

```console
busybar api streaming status-ws
```

If you provide a `--frame-dir <dir>` option, `busybar` as a side effect, will decode the streamed frames and write
them as images to the given folder.

By default, front frames are rendered with a black raster to mimic the
matrix display of the actual device. This can be disabled by providing the `--no-image-raster` flag.

```console
busybar api streaming status-ws --frame-dir ./frames
```

Reported events carry the frame inline as base64, in the same `--image-format` the files use.

```console
busybar --image-format jpg -o json api streaming status-ws
```
