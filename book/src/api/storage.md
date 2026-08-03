# storage

## write

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();

let data = std::fs::read("test.png").unwrap();
client.storage().write("/ext/test.png", data).await?;

# Ok(())
# }
```

## read

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let data = client.storage().read("/ext/test.png").await?;

# Ok(())
# }
```

## list

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let entries = client.storage().list("/ext").await?;

# Ok(())
# }
```

## remove

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.storage().remove("/ext/test.png").await?;

# Ok(())
# }
```

## mkdir

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.storage().mkdir("/ext/sub").await?;

# Ok(())
# }
```

## rename

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

## status

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let status = client.storage().status().await?;

# Ok(())
# }
```
