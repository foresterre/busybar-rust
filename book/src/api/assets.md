# assets

## upload

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();

let data = std::fs::read("data.png").unwrap();
client.assets().upload("my_app", "data.png", data).await?;

# Ok(())
# }
```

## delete

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.assets().delete("my_app").await?;

# Ok(())
# }
```

## play

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::assets::PlayAudio;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client
    .assets()
    .play(&PlayAudio::stock("my_app", "shared/beep.snd")?)
    .await?;

# Ok(())
# }
```

## stop

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.assets().stop().await?;

# Ok(())
# }
```

## draw

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::model::assets::{DisplayElement, DisplayElements, Font, TextElement};
use busylib::types::color::Color;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();

let elements = DisplayElements::new("my_app")?
    .led_notification_color(Color::RED)
    .element(
        DisplayElement::builder("0")?
            .timeout_secs(5)
            .text(TextElement::new("busy", Font::Large)?),
    );
client.assets().draw(&elements).await?;

# Ok(())
# }
```

## clear

```rust
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
use busylib::ClientBuilder;
use busylib::types::app_name::AppName;

let client = ClientBuilder::new("http://10.0.4.20")?.build_reqwest();
client.assets().clear(Some(AppName::new("my_app")?)).await?;

# Ok(())
# }
```
