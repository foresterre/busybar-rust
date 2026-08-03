# Account

## info

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let info = client.account().info().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api account info
```

## status

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let status = client.account().status().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api account status
```

## backend

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let backend = client.account().backend().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api account backend
```
