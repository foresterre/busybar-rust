# smart_home

## pairing

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let pairing = client.smart_home().pairing().await?;

# Ok(())
# }
```

## start_pairing

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let payload = client.smart_home().start_pairing().await?;

# Ok(())
# }
```

## erase_pairings

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.smart_home().erase_pairings().await?;

# Ok(())
# }
```

## switch

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
let switch = client.smart_home().switch().await?;

# Ok(())
# }
```

## set_switch

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
