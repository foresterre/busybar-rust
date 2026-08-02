use busylib::model::audio::PlayAudio;
use busylib::model::busy::BusyProfileSlot;
use busylib::model::display::{DisplayElement, DisplayElements, Font, Screen, TextElement};
use busylib::model::input::Key;
use busylib::model::settings::HttpAccess;
use busylib::model::system::TransportType;
use busylib::types::brightness::Brightness;
use busylib::types::color::Color;
use busylib::types::volume::Volume;
use busylib::{Client, ClientBuilder, Error, ReqwestHttpTransport};
use serde_json::json;
use wiremock::matchers::{body_bytes, body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TOKEN: &str = "example_busybar_tok";

async fn device() -> (MockServer, Client<ReqwestHttpTransport>) {
    let server = MockServer::start().await;

    let client = ClientBuilder::new(server.uri())
        .unwrap()
        .token(TOKEN)
        .unwrap()
        .build_reqwest();

    (server, client)
}

fn ok() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"result": "OK"}))
}

#[tokio::test]
async fn gets_account_info_with_a_bearer_token() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/account/info"))
        .and(header("authorization", format!("Bearer {TOKEN}").as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "linked": true,
            "id": "12345678-9abc-def0-1234-56789abcdef0",
            "email": "name@example.com"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let info = client.account().info().await.unwrap();

    assert!(info.is_linked());
    assert_eq!(info.email.as_deref(), Some("name@example.com"));
    assert_eq!(info.user_id, None);
}

#[tokio::test]
async fn unwraps_single_field_responses() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"api_semver": "25.0.0"})))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/busybar/transport"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"type": "usb"})))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/busybar/time"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"timestamp": "2025-10-02T14:30:45+04:00"})),
        )
        .mount(&server)
        .await;

    assert_eq!(client.system().version().await.unwrap(), "25.0.0");

    assert_eq!(
        client.system().transport().await.unwrap(),
        TransportType::Usb
    );

    assert_eq!(
        client.time().now().await.unwrap().as_str(),
        "2025-10-02T14:30:45+04:00"
    );
}

#[tokio::test]
async fn draws_on_the_display() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/display/draw"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "application_name": "my_app",
            "led_notification_color": "#FF0000FF",
            "elements": [{
                "id": "0",
                "timeout": 5,
                "type": "text",
                "text": "busy",
                "font": "large"
            }]
        })))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    let elements = DisplayElements::new("my_app")
        .unwrap()
        .led_notification_color(Color::RED)
        .element(
            DisplayElement::builder("0")
                .unwrap()
                .timeout_secs(5)
                .text(TextElement::new("busy", Font::Large).unwrap()),
        );

    client.display().draw(&elements).await.unwrap();
}

#[tokio::test]
async fn clears_the_display_for_one_app() {
    let (server, client) = device().await;

    Mock::given(method("DELETE"))
        .and(path("/busybar/display/draw"))
        .and(query_param("application_name", "my_app"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.display().clear_app("my_app").await.unwrap();
}

#[tokio::test]
async fn reads_and_sets_brightness() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/display/brightness"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"value": "auto"})))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/busybar/display/brightness"))
        .and(query_param("value", "40"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    assert_eq!(
        client.display().brightness().await.unwrap(),
        Brightness::Auto
    );

    client
        .display()
        .set_brightness(Brightness::level(40).unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn fetches_a_screen_frame_as_bytes() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/screen"))
        .and(query_param("display", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(b"BM\x00\x01".to_vec(), "image/bmp"))
        .expect(1)
        .mount(&server)
        .await;

    let frame = client.display().frame(Screen::Back).await.unwrap();

    assert_eq!(frame.as_ref(), b"BM\x00\x01");
}

#[tokio::test]
async fn uploads_an_asset_as_a_binary_body() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/assets/upload"))
        .and(query_param("application_name", "my_app"))
        .and(query_param("file", "data.png"))
        .and(header("content-type", "application/octet-stream"))
        .and(body_bytes(b"\x89PNG".to_vec()))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client
        .assets()
        .upload("my_app", "data.png", b"\x89PNG".to_vec())
        .await
        .unwrap();
}

#[tokio::test]
async fn plays_and_stops_audio() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/audio/play"))
        .and(body_json(
            json!({"application_name": "my_app", "stock_path": "shared/beep.snd"}),
        ))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/busybar/audio/play"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client
        .audio()
        .play(&PlayAudio::stock("my_app", "shared/beep.snd").unwrap())
        .await
        .unwrap();

    client.audio().stop().await.unwrap();
}

#[tokio::test]
async fn sets_the_volume_silently() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/audio/volume"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"volume": 50})))
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/busybar/audio/volume"))
        .and(query_param("volume", "35"))
        .and(query_param("silent", "1"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    assert_eq!(
        client.audio().volume().await.unwrap(),
        Volume::new(50).unwrap()
    );

    client
        .audio()
        .set_volume_silently(Volume::new(35).unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn sends_an_input_event() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/input"))
        .and(query_param("key", "ok"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.input().press(Key::Ok).await.unwrap();
}

#[tokio::test]
async fn walks_the_storage_endpoints() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/storage/write"))
        .and(query_param("path", "/ext/test.png"))
        .and(body_bytes(b"data".to_vec()))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/busybar/storage/read"))
        .and(query_param("path", "/ext/test.png"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(b"data".to_vec(), "application/octet-stream"),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/busybar/storage/list"))
        .and(query_param("path", "/ext"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "list": [{"type": "file", "name": "test.png", "size": 4}, {"type": "dir", "name": "sub"}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/busybar/storage/rename"))
        .and(query_param("path", "/ext/test.png"))
        .and(query_param("new_path", "/ext/sub/test.png"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("DELETE"))
        .and(path("/busybar/storage/remove"))
        .and(query_param("path", "/ext/sub/test.png"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/busybar/storage/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"used_bytes": 123456, "free_bytes": 654321, "total_bytes": 777777}),
        ))
        .expect(1)
        .mount(&server)
        .await;

    let storage = client.storage();

    storage
        .write("/ext/test.png", b"data".to_vec())
        .await
        .unwrap();

    assert_eq!(
        storage.read("/ext/test.png").await.unwrap().as_ref(),
        b"data"
    );

    let entries = storage.list("/ext").await.unwrap();

    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].size(), Some(4));

    storage
        .rename("/ext/test.png", "/ext/sub/test.png")
        .await
        .unwrap();

    storage.remove("/ext/sub/test.png").await.unwrap();

    assert_eq!(storage.status().await.unwrap().total_bytes, Some(777_777));
}

#[tokio::test]
async fn reads_a_busy_profile_from_its_slot() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/busy/profiles/custom"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "sort_order": -1,
            "title": "study",
            "id": "00000000-0000-0000-0000-000000000000",
            "timer_settings": {"type": "SIMPLE", "total_time_ms": 300000},
            "busy_bar_settings": {
                "theme": "on_air",
                "show_work_phase_only": false,
                "trigger_smart_home": true
            },
            "profile_timestamp_ms": 1761582532251u64
        })))
        .expect(1)
        .mount(&server)
        .await;

    let profile = client
        .busy()
        .profile(BusyProfileSlot::Custom)
        .await
        .unwrap();

    assert_eq!(profile.title, "study");
}

#[tokio::test]
async fn sets_http_access_with_a_key() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/access"))
        .and(query_param("mode", "key"))
        .and(query_param("key", "12345678"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client
        .settings()
        .set_http_access(&HttpAccess::Key("12345678".parse().unwrap()))
        .await
        .unwrap();
}

#[tokio::test]
async fn sets_http_access_without_a_key() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/access"))
        .and(query_param("mode", "disabled"))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client
        .settings()
        .set_http_access(&HttpAccess::Disabled)
        .await
        .unwrap();

    let request = &server.received_requests().await.unwrap()[0];

    assert_eq!(request.url.query(), Some("mode=disabled"));
}

#[tokio::test]
async fn renames_the_device() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/name"))
        .and(body_json(json!({"name": "BUSY bar"})))
        .respond_with(ok())
        .expect(1)
        .mount(&server)
        .await;

    client.settings().set_name("BUSY bar").await.unwrap();
}

#[tokio::test]
async fn dumps_the_log_to_a_named_file() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/log_dump"))
        .and(query_param("filename", "dump"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"result": "OK", "path": "/ext/dump.txt"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    assert_eq!(
        client.system().dump_log_as("dump").await.unwrap(),
        "/ext/dump.txt"
    );
}

#[tokio::test]
async fn accepts_an_empty_success_body() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/update/abort_download"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    client.updater().abort_download().await.unwrap();
}

#[tokio::test]
async fn keeps_a_path_prefix_from_the_base_url() {
    let server = MockServer::start().await;

    let client = ClientBuilder::new(format!("{}/proxy", server.uri()))
        .unwrap()
        .build_reqwest();

    Mock::given(method("GET"))
        .and(path("/proxy/busybar/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"api_semver": "25.0.0"})))
        .expect(1)
        .mount(&server)
        .await;

    assert_eq!(client.system().version().await.unwrap(), "25.0.0");
}

#[tokio::test]
async fn reports_api_errors_with_their_code() {
    let (server, client) = device().await;

    Mock::given(method("POST"))
        .and(path("/busybar/input"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(json!({"error": "Invalid parameter", "code": 400})),
        )
        .mount(&server)
        .await;

    let error = client.input().press(Key::Up).await.unwrap_err();

    let Error::Api {
        status,
        error: ref api_error,
        ..
    } = error
    else {
        panic!("Expected an API error, got {error:?}");
    };

    assert_eq!(status, 400);
    assert_eq!(api_error.code, Some(400));
    assert_eq!(
        error.to_string(),
        "POST /busybar/input was rejected with 400 Bad Request: Invalid parameter (code 400)"
    );
}

#[tokio::test]
async fn reports_unauthorized_requests() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/status"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({"error": "Unauthorized"})))
        .mount(&server)
        .await;

    let error = client.system().status().await.unwrap_err();

    assert!(error.is_unauthorized());
    assert_eq!(
        error.api_error().map(|error| error.error.as_str()),
        Some("Unauthorized")
    );
}

#[tokio::test]
async fn reports_non_json_failures_with_a_body_snippet() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/wifi/status"))
        .respond_with(ResponseTemplate::new(503).set_body_string("<html>gateway down</html>"))
        .mount(&server)
        .await;

    let error = client.wifi().status().await.unwrap_err();

    assert!(matches!(error, Error::UnexpectedStatus { .. }));
    assert_eq!(
        error.to_string(),
        "GET /busybar/wifi/status returned an unexpected 503 Service Unavailable response: <html>gateway down</html>"
    );
}

#[tokio::test]
async fn reports_responses_that_do_not_match_the_api() {
    let (server, client) = device().await;

    Mock::given(method("GET"))
        .and(path("/busybar/account/backend"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"server_url": "default"})))
        .mount(&server)
        .await;

    let error = client.account().backend().await.unwrap_err();

    assert!(matches!(error, Error::Decode { .. }));
    assert!(
        error.to_string().starts_with(
            "GET /busybar/account/backend returned a response that does not match the API:"
        ),
        "unexpected message: {error}"
    );
    assert!(
        std::error::Error::source(&error)
            .unwrap()
            .to_string()
            .starts_with("missing field `client_cert_type`")
    );
}

#[tokio::test]
async fn gives_up_on_a_slow_device_when_a_timeout_is_set() {
    let server = MockServer::start().await;

    let client = ClientBuilder::new(server.uri())
        .unwrap()
        .timeout(std::time::Duration::from_millis(50))
        .build_reqwest();

    Mock::given(method("GET"))
        .and(path("/busybar/version"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_delay(std::time::Duration::from_secs(5))
                .set_body_json(json!({"api_semver": "25.0.0"})),
        )
        .mount(&server)
        .await;

    let error = client.system().version().await.unwrap_err();

    assert!(matches!(error, Error::Transport { .. }), "{error:?}");
    assert_eq!(
        error.to_string(),
        "GET /busybar/version unable to reach device"
    );
}

#[tokio::test]
async fn reports_transport_failures() {
    let client = Client::new(ReqwestHttpTransport::new(), "http://127.0.0.1:1").unwrap();

    let error = client.system().version().await.unwrap_err();

    assert!(matches!(error, Error::Transport { .. }));
    assert_eq!(
        error.to_string(),
        "GET /busybar/version unable to reach device"
    );
    assert!(std::error::Error::source(&error).is_some());
}

#[tokio::test]
async fn rejects_invalid_values_before_sending_a_request() {
    let (_server, client) = device().await;

    let error = client.storage().read("/etc/passwd").await.unwrap_err();

    assert!(matches!(error, Error::Value(_)));
    assert_eq!(
        error.to_string(),
        "invalid storage path `/etc/passwd`: expected `/ext` optionally followed by `/` separated segments of [a-zA-Z0-9._-]"
    );
}
