# Wifi

## status

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let status = client.wifi().status().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api wifi status
```
