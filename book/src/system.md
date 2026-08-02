# System

## version

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let version = client.system().version().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar system version
```

## transport

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let transport = client.system().transport().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar system transport
```

## status

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let status = client.system().status().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar system status
```

## status_device

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let device = client.system().status_device().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar system status-device
```

## status_firmware

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let firmware = client.system().status_firmware().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar system status-firmware
```

## status_system

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let system = client.system().status_system().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar system status-system
```

## status_power

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let power = client.system().status_power().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar system status-power
```

## log_dump

<h3>API client</h3>

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

<h3>CLI</h3>

```console
busybar system log-dump --filename dump
```
