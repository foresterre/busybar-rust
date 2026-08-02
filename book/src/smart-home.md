# SmartHome

## pairing

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let pairing = client.smart_home().pairing().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar smart-home pairing
```

## start_pairing

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let payload = client.smart_home().start_pairing().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar smart-home start-pairing
```

## erase_pairings

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.smart_home().erase_pairings().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar smart-home erase-pairings
```

## switch

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let switch = client.smart_home().switch().await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar smart-home switch
```

## set_switch

<h3>API client</h3>

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::smart_home::{SmartHomeSwitchState, SwitchStartup};

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client
    .smart_home()
    .set_switch(&SmartHomeSwitchState::on().startup(SwitchStartup::Last))
    .await?;

# Ok(())
# }
```

<h3>CLI</h3>

```console
busybar smart-home set-switch on --startup last
```
