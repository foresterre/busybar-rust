# time

## now

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let now = client.time().now().await?;

# Ok(())
# }
```

## set_timestamp

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

## timezone

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let timezone = client.time().timezone().await?;

# Ok(())
# }
```

## set_timezone

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.time().set_timezone("Berlin").await?;

# Ok(())
# }
```

## tzlist

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let timezones = client.time().tzlist().await?;

# Ok(())
# }
```
