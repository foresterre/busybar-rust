# capture-frames

Decodes the frames streamed by the device and writes them as images to the given folder, until interrupted with
ctrl-c. The `--screen` option selects which screen is captured.

⚠️ The `--screen back` doesn't work yet: the stream does not seem to contain the back frames

```console
busybar capture-frames ./frames
```

```console
busybar capture-frames --screen front ./frames
```

Frames are written in the `--image-format` given, or as the raw device bytes with `--image-format raw`.

```console
busybar --image-format jpg capture-frames ./frames
```

By default, front frames are rendered with a black raster to mimic the matrix display of the actual device. This can
be disabled by providing the `--no-image-raster` flag.

```console
busybar capture-frames --no-image-raster ./frames
```
