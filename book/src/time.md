# Time

## now

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let now = client.time().now().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api time now
```

## set_timestamp

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client
    .time()
    .set_timestamp("2025-10-02T14:30:45+04:00")
    .await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api time set-timestamp 2025-10-02T14:30:45+04:00
```

## timezone

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let timezone = client.time().timezone().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api time timezone
```

## set_timezone

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.time().set_timezone("Berlin").await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api time set-timezone Berlin
```

## tzlist

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let timezones = client.time().tzlist().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api time tzlist
```
