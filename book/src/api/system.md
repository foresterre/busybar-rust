# system

## version

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let version = client.system().version().await?;

# Ok(())
# }
```

## transport

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let transport = client.system().transport().await?;

# Ok(())
# }
```

## status

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let status = client.system().status().await?;

# Ok(())
# }
```

## status_device

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let device = client.system().status_device().await?;

# Ok(())
# }
```

## status_firmware

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let firmware = client.system().status_firmware().await?;

# Ok(())
# }
```

## status_system

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let system = client.system().status_system().await?;

# Ok(())
# }
```

## status_power

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let power = client.system().status_power().await?;

# Ok(())
# }
```

## log_dump

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::types::log_name::LogName;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let path = client
    .system()
    .log_dump(Some(LogName::new("dump")?))
    .await?;

# Ok(())
# }
```
