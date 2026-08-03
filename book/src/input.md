# Input

## press

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::input::Key;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.input().press(Key::Ok).await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar api input press ok
```
