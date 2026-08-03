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

## Cast

<script src="https://asciinema.org/a/4Fitxb74bAGJDs46.js" id="asciicast-4Fitxb74bAGJDs46" async="true"></script>

**Frames captured**

![front-000001](../assets/cli/capture-frames-demo/front-000001.png)
![front-000002](../assets/cli/capture-frames-demo/front-000002.png)
![front-000003](../assets/cli/capture-frames-demo/front-000003.png)
![front-000004](../assets/cli/capture-frames-demo/front-000004.png)
![front-000005](../assets/cli/capture-frames-demo/front-000005.png)
![front-000006](../assets/cli/capture-frames-demo/front-000006.png)
![front-000007](../assets/cli/capture-frames-demo/front-000007.png)
![front-000008](../assets/cli/capture-frames-demo/front-000008.png)
![front-000009](../assets/cli/capture-frames-demo/front-000009.png)
![front-000010](../assets/cli/capture-frames-demo/front-000010.png)

