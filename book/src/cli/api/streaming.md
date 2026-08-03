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

Reported events carry the frame inline as base64. The format matches the value of the global `--image-format` option.
Note that in "text" mode the actual frame data isn't printed. You can get the frames in conventional
image formats via [`busybar capture-frames`](../capture-frames.md), or set the output format to `json`.

```console
busybar --image-format jpg --output-format json api streaming status-ws
```

Front frames are rendered with a black raster to mimic the matrix display of the actual device. 
This can be disabled by providing the `--no-image-raster` flag.

```console
busybar --output-format json api streaming status-ws --no-image-raster
```
