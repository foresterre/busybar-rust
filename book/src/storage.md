# Storage

## write

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();

let data = std::fs::read("test.png").unwrap();
client.storage().write("/ext/test.png", data).await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar storage write /ext/test.png --file ./test.png
```

## read

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let data = client.storage().read("/ext/test.png").await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar storage read /ext/test.png --output ./test.png
```

## list

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let entries = client.storage().list("/ext").await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar storage list /ext
```

## remove

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.storage().remove("/ext/test.png").await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar storage remove /ext/test.png
```

## mkdir

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.storage().mkdir("/ext/sub").await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar storage mkdir /ext/sub
```

## rename

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client
    .storage()
    .rename("/ext/test.png", "/ext/sub/test.png")
    .await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar storage rename /ext/test.png /ext/sub/test.png
```

## status

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let status = client.storage().status().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar storage status
```
