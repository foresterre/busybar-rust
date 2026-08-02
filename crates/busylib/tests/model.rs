use busylib::{
    Align, AnimationElement, AutoupdateSettings, BleState, BleStatus, BusyBarSettings, BusyProfile,
    BusySnapshot, Color, CountdownDirection, CountdownElement, DisplayElements, Element, Font,
    ImageElement, IntervalSettings, Lifetime, MqttStatus, Opacity, PlayAudio, Priority,
    RectangleElement, Screen, ShowHours, SmartHomePairing, SmartHomeSwitch, Snapshot, StorageEntry,
    SwitchStartup, TextElement, TimeOfDay, TimerSettings, TimerState, UpdateStatus, WifiSecurity,
    WifiState, WifiStatus,
};
use serde_json::json;

#[test]
fn serializes_display_elements_like_the_api_example() {
    let elements = DisplayElements::new("my_app")
        .unwrap()
        .priority(Priority::new(50).unwrap())
        .led_notification_color(Color::RED)
        .element(
            Element::builder("0")
                .unwrap()
                .timeout_secs(10)
                .align(Align::Center)
                .at(36, 10)
                .screen(Screen::Front)
                .text(
                    TextElement::new("Hello, World! Long text", Font::Normal)
                        .unwrap()
                        .color(Color::WHITE)
                        .width(72)
                        .scroll_rate(1000)
                        .scroll_start_delay_ms(1000)
                        .scroll_repeat_delay_ms(2500),
                ),
        )
        .element(
            Element::builder("2")
                .unwrap()
                .timeout_secs(6)
                .at(0, 0)
                .screen(Screen::Back)
                .image(ImageElement::asset("data.png").unwrap()),
        );

    assert_eq!(
        serde_json::to_value(&elements).unwrap(),
        json!({
            "application_name": "my_app",
            "priority": 50,
            "led_notification_color": "#FF0000FF",
            "elements": [
                {
                    "id": "0",
                    "timeout": 10,
                    "align": "center",
                    "x": 36,
                    "y": 10,
                    "display": "front",
                    "type": "text",
                    "text": "Hello, World! Long text",
                    "font": "normal",
                    "color": "#FFFFFFFF",
                    "width": 72,
                    "scroll_rate": 1000,
                    "scroll_start_delay": 1000,
                    "scroll_repeat_delay": 2500
                },
                {
                    "id": "2",
                    "timeout": 6,
                    "x": 0,
                    "y": 0,
                    "display": "back",
                    "type": "image",
                    "path": "data.png"
                }
            ]
        })
    );
}

#[test]
fn omits_absent_element_fields() {
    let elements = DisplayElements::new("my_app").unwrap().element(
        Element::builder("bare")
            .unwrap()
            .text(TextElement::new("hi", Font::Tiny).unwrap()),
    );

    assert_eq!(
        serde_json::to_value(&elements).unwrap(),
        json!({
            "application_name": "my_app",
            "elements": [{"id": "bare", "type": "text", "text": "hi", "font": "tiny"}]
        })
    );
}

#[test]
fn round_trips_every_element_kind() {
    let elements = DisplayElements::new("my_app")
        .unwrap()
        .element(
            Element::builder("text")
                .unwrap()
                .text(TextElement::new("hi", Font::ExtraLarge).unwrap()),
        )
        .element(
            Element::builder("image").unwrap().image(
                ImageElement::stock("shared/icon.png")
                    .unwrap()
                    .opacity(Opacity::new(40).unwrap()),
            ),
        )
        .element(
            Element::builder("animation")
                .unwrap()
                .display_until(1_761_582_532)
                .animation(
                    AnimationElement::asset("anim/spin.gif")
                        .unwrap()
                        .repeat(true)
                        .await_previous_end(false)
                        .section("default"),
                ),
        )
        .element(
            Element::builder("countdown").unwrap().countdown(
                CountdownElement::new(
                    1_761_582_532,
                    CountdownDirection::TimeLeft,
                    ShowHours::WhenNonZero,
                )
                .color(Color::BLUE),
            ),
        )
        .element(
            Element::builder("rectangle").unwrap().rectangle(
                RectangleElement::new(20, 10)
                    .radius(2)
                    .horizontal_gradient(Color::WHITE, Color::TRANSPARENT)
                    .border(1, Color::RED),
            ),
        );

    let json = serde_json::to_value(&elements).unwrap();
    let parsed: DisplayElements = serde_json::from_value(json).unwrap();

    assert_eq!(parsed, elements);
}

#[test]
fn serializes_animation_loop_as_loop() {
    let element = Element::builder("a").unwrap().animation(
        AnimationElement::stock("shared/wave.anim")
            .unwrap()
            .repeat(true),
    );

    let json = serde_json::to_value(&element).unwrap();

    assert_eq!(json["loop"], json!(true));
    assert_eq!(json["stock_path"], json!("shared/wave.anim"));
    assert_eq!(json["type"], json!("animation"));
}

#[test]
fn serializes_display_until_as_string() {
    let lifetime = Lifetime::display_until(1_761_582_532);

    assert_eq!(
        serde_json::to_value(lifetime).unwrap(),
        json!({"display_until": "1761582532"})
    );

    assert_eq!(
        serde_json::from_value::<Lifetime>(json!({"timeout": 5})).unwrap(),
        Lifetime::timeout_secs(5)
    );
}

#[test]
fn serializes_audio_sources() {
    assert_eq!(
        serde_json::to_value(PlayAudio::asset("my_app", "data.snd").unwrap()).unwrap(),
        json!({"application_name": "my_app", "path": "data.snd"})
    );

    assert_eq!(
        serde_json::to_value(PlayAudio::stock("my_app", "shared/beep.snd").unwrap()).unwrap(),
        json!({"application_name": "my_app", "stock_path": "shared/beep.snd"})
    );
}

#[test]
fn parses_interval_snapshot_and_keeps_the_nested_tag() {
    let payload = json!({
        "snapshot": {
            "type": "INTERVAL",
            "card_id": "00000000-0000-0000-0000-000000000000",
            "current_interval": 1,
            "current_interval_time_total_ms": 60000,
            "current_interval_time_left_ms": 42690,
            "is_paused": false,
            "interval_settings": {
                "type": "INTERVAL",
                "interval_work_ms": 120000,
                "interval_rest_ms": 60000,
                "interval_work_cycles_count": 3,
                "is_autostart_enabled": false
            },
            "busy_bar_settings": {
                "theme": "on_air",
                "show_work_phase_only": false,
                "trigger_smart_home": true
            }
        },
        "snapshot_timestamp_ms": 1761582532251u64
    });

    let snapshot: BusySnapshot = serde_json::from_value(payload.clone()).unwrap();

    let TimerState::Interval {
        current_interval,
        interval_settings,
        ..
    } = &snapshot.snapshot.state
    else {
        panic!(
            "expected an interval snapshot, got {:?}",
            snapshot.snapshot.state
        );
    };

    assert_eq!(*current_interval, 1);
    assert_eq!(interval_settings.interval_work_ms, 120_000);
    assert_eq!(serde_json::to_value(&snapshot).unwrap(), payload);
}

#[test]
fn rejects_a_nested_non_interval_settings_object() {
    let payload = json!({
        "snapshot": {
            "type": "INTERVAL",
            "card_id": "x",
            "current_interval": 1,
            "current_interval_time_total_ms": 1,
            "current_interval_time_left_ms": 1,
            "is_paused": false,
            "interval_settings": {"type": "SIMPLE", "total_time_ms": 1},
            "busy_bar_settings": {
                "theme": "on_air",
                "show_work_phase_only": false,
                "trigger_smart_home": true
            }
        },
        "snapshot_timestamp_ms": 1
    });

    let error = serde_json::from_value::<BusySnapshot>(payload).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("expected INTERVAL timer settings"),
        "unexpected error: {error}"
    );
}

#[test]
fn round_trips_a_not_started_snapshot() {
    let snapshot = BusySnapshot {
        snapshot: Snapshot {
            state: TimerState::NotStarted,
            busy_bar_settings: BusyBarSettings {
                theme: "on_air".to_owned(),
                show_work_phase_only: false,
                trigger_smart_home: true,
            },
        },
        snapshot_timestamp_ms: 1,
    };

    let json = serde_json::to_value(&snapshot).unwrap();

    assert_eq!(json["snapshot"]["type"], json!("NOT_STARTED"));
    assert_eq!(
        serde_json::from_value::<BusySnapshot>(json).unwrap(),
        snapshot
    );
}

#[test]
fn round_trips_a_busy_profile() {
    let profile = BusyProfile {
        id: "00000000-0000-0000-0000-000000000000".to_owned(),
        title: "study".to_owned(),
        sort_order: -1,
        timer_settings: TimerSettings::interval(IntervalSettings {
            interval_work_ms: 120_000,
            interval_rest_ms: 60_000,
            interval_work_cycles_count: 3,
            is_autostart_enabled: false,
        }),
        busy_bar_settings: BusyBarSettings {
            theme: "on_air".to_owned(),
            show_work_phase_only: false,
            trigger_smart_home: true,
        },
        profile_timestamp_ms: 1_761_582_532_251,
    };

    let json = serde_json::to_value(&profile).unwrap();

    assert_eq!(json["timer_settings"]["type"], json!("INTERVAL"));
    assert_eq!(json["timer_settings"]["interval_work_ms"], json!(120_000));
    assert_eq!(
        serde_json::from_value::<BusyProfile>(json).unwrap(),
        profile
    );
}

#[test]
fn parses_storage_entries() {
    let entries: Vec<StorageEntry> = serde_json::from_value(json!([
        {"type": "file", "name": "test.png", "size": 65535},
        {"type": "dir", "name": "assets"}
    ]))
    .unwrap();

    assert_eq!(entries[0].name(), "test.png");
    assert_eq!(entries[0].size(), Some(65535));
    assert!(!entries[0].is_dir());

    assert!(entries[1].is_dir());
    assert_eq!(entries[1].size(), None);
}

#[test]
fn parses_a_connected_wifi_status() {
    let status: WifiStatus = serde_json::from_value(json!({
        "state": "connected",
        "ssid": "home",
        "bssid": "EC:5A:00:0B:55:1D",
        "channel": 3,
        "rssi": -43,
        "security": "WPA2/WPA3",
        "ip_config": {"ip_method": "dhcp", "ip_type": "ipv4", "address": "192.168.50.5"}
    }))
    .unwrap();

    assert!(status.is_connected());
    assert_eq!(status.security, Some(WifiSecurity::Wpa2Wpa3));
    assert_eq!(
        status
            .ip_config
            .and_then(|config| config.address)
            .as_deref(),
        Some("192.168.50.5")
    );
}

#[test]
fn parses_a_disconnected_wifi_status() {
    let status: WifiStatus = serde_json::from_value(json!({"state": "disconnected"})).unwrap();

    assert_eq!(status.state, WifiState::Disconnected);
    assert!(!status.is_connected());
    assert_eq!(status.ssid, None);
}

#[test]
fn keeps_unknown_enum_values() {
    let status: WifiStatus = serde_json::from_value(json!({"state": "scanning"})).unwrap();

    assert_eq!(status.state, WifiState::Unrecognized("scanning".to_owned()));

    let ble: BleStatus = serde_json::from_value(json!({"status": "internal error"})).unwrap();

    assert_eq!(ble.status, BleState::InternalError);

    assert_eq!(
        serde_json::from_value::<MqttStatus>(json!("reconnecting")).unwrap(),
        MqttStatus::Unknown("reconnecting".to_owned())
    );
}

#[test]
fn parses_update_status_progress() {
    let status: UpdateStatus = serde_json::from_value(json!({
        "install": {
            "is_allowed": true,
            "event": "action_progress",
            "action": "download",
            "status": "ok",
            "detail": "",
            "download": {
                "speed_bytes_per_sec": 1024,
                "received_bytes": 512,
                "total_bytes": 2048
            }
        },
        "check": {"available_version": "1.2.3", "event": "stop", "status": "available"}
    }))
    .unwrap();

    let install = status.install.unwrap();

    assert_eq!(install.download.unwrap().fraction(), Some(0.25));
    assert_eq!(status.check.unwrap().available_version(), Some("1.2.3"));
}

#[test]
fn treats_an_empty_available_version_as_none() {
    let status: UpdateStatus =
        serde_json::from_value(json!({"check": {"available_version": ""}})).unwrap();

    assert_eq!(status.check.unwrap().available_version(), None);
}

#[test]
fn serializes_only_the_autoupdate_fields_that_are_set() {
    let settings = AutoupdateSettings::new().enabled(true);

    assert_eq!(
        serde_json::to_value(&settings).unwrap(),
        json!({"is_enabled": true})
    );

    let settings = settings.window(
        TimeOfDay::new("00:00").unwrap(),
        TimeOfDay::new("08:00").unwrap(),
    );

    assert_eq!(
        serde_json::to_value(&settings).unwrap(),
        json!({"is_enabled": true, "interval_start": "00:00", "interval_end": "08:00"})
    );
}

#[test]
fn serializes_the_smart_home_switch() {
    assert_eq!(
        serde_json::to_value(SmartHomeSwitch::on()).unwrap(),
        json!({"state": true})
    );

    assert_eq!(
        serde_json::to_value(SmartHomeSwitch::off().startup(SwitchStartup::Last)).unwrap(),
        json!({"state": false, "startup": "last"})
    );
}

#[test]
fn parses_the_smart_home_pairing_payload() {
    let pairing: SmartHomePairing = serde_json::from_value(json!({
        "available_until": "1769437579000",
        "qr_code": "MT:YNDA0-O913..VV7I000",
        "manual_code": "1155-360-0377"
    }))
    .unwrap();

    assert_eq!(pairing.available_until, Some(1_769_437_579_000));
}
