# Ble

## enable

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.ble().enable().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar ble enable
```

## disable

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.ble().disable().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar ble disable
```

## remove_pairing

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.ble().remove_pairing().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar ble remove-pairing
```

## status

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let status = client.ble().status().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar ble status
```
