use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use busylib::types::path_prefix::PathPrefix;
use busylib::{
    ApiPrefix, BoxFuture, Client, ClientBuilder, Error, HttpTransport, HttpTransportError,
    HttpTransportResult, Timeout,
};
use bytes::Bytes;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::{Method, Request, Response};

#[derive(Default)]
struct StubTransport {
    requests: Mutex<Vec<Request<Bytes>>>,
    responses: Mutex<VecDeque<Response<Bytes>>>,
}

impl StubTransport {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn queue(&self, status: u16, content_type: &str, body: &'static [u8]) {
        let response = Response::builder()
            .status(status)
            .header(CONTENT_TYPE, content_type)
            .body(Bytes::from_static(body))
            .expect("valid response");

        self.responses.lock().unwrap().push_back(response);
    }

    fn queue_json(&self, status: u16, body: &'static str) {
        self.queue(status, "application/json", body.as_bytes());
    }

    fn requests(&self) -> Vec<Request<Bytes>> {
        std::mem::take(&mut self.requests.lock().unwrap())
    }
}

impl HttpTransport for StubTransport {
    fn execute(&self, request: Request<Bytes>) -> BoxFuture<'_, HttpTransportResult> {
        self.requests.lock().unwrap().push(request);

        let queued = self.responses.lock().unwrap().pop_front();

        Box::pin(async move { queued.ok_or_else(|| HttpTransportError::new("no response queued")) })
    }
}

#[tokio::test]
async fn drives_the_api_through_a_shared_transport() {
    let transport = StubTransport::new();
    transport.queue_json(200, r#"{"api_semver":"25.0.0"}"#);

    let client = ClientBuilder::new("http://busy.local")
        .unwrap()
        .token("secret")
        .unwrap()
        .build(Arc::clone(&transport));

    assert_eq!(client.system().version().await.unwrap(), "25.0.0");

    let requests = transport.requests();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method(), Method::GET);
    assert_eq!(
        requests[0].uri().to_string(),
        "http://busy.local/api/version"
    );
    assert_eq!(requests[0].headers()[AUTHORIZATION], "Bearer secret");
    assert!(requests[0].body().is_empty());
    assert!(!requests[0].headers().contains_key(CONTENT_TYPE));
}

#[tokio::test]
async fn drives_the_api_through_a_boxed_transport() {
    let transport = StubTransport::new();
    transport.queue_json(200, r#"{"result":"OK"}"#);

    let boxed: Box<dyn HttpTransport> = Box::new(Arc::clone(&transport));
    let client = ClientBuilder::new("http://busy.local")
        .unwrap()
        .build(boxed);

    client.ble().enable().await.unwrap();

    let requests = transport.requests();

    assert_eq!(requests[0].method(), Method::POST);
    assert_eq!(
        requests[0].uri().to_string(),
        "http://busy.local/api/ble/enable"
    );
}

#[tokio::test]
async fn leaves_out_the_authorization_header_without_a_token() {
    let transport = StubTransport::new();
    transport.queue_json(200, r#"{"status":"connected"}"#);

    let client = Client::new(Arc::clone(&transport), "http://busy.local").unwrap();

    client.account().status().await.unwrap();

    assert!(
        !transport.requests()[0]
            .headers()
            .contains_key(AUTHORIZATION)
    );
}

#[tokio::test]
async fn percent_encodes_query_parameters() {
    let transport = StubTransport::new();
    transport.queue(200, "application/octet-stream", b"payload");

    let client = Client::new(Arc::clone(&transport), "http://busy.local/").unwrap();

    let data = client.storage().read("/ext/dir/test.png").await.unwrap();

    assert_eq!(data.as_ref(), b"payload");
    assert_eq!(
        transport.requests()[0].uri().to_string(),
        "http://busy.local/api/storage/read?path=%2Fext%2Fdir%2Ftest.png"
    );
}

#[tokio::test]
async fn sends_uploads_as_binary_bodies() {
    let transport = StubTransport::new();
    transport.queue_json(200, r#"{"result":"OK"}"#);

    let client = Client::new(Arc::clone(&transport), "http://busy.local").unwrap();

    client
        .updater()
        .update(Bytes::from_static(b"tarball"))
        .await
        .unwrap();

    let requests = transport.requests();

    assert_eq!(
        requests[0].headers()[CONTENT_TYPE],
        "application/octet-stream"
    );
    assert_eq!(requests[0].body().as_ref(), b"tarball");
}

#[tokio::test]
async fn leaves_out_the_timeout_extension_by_default() {
    let transport = StubTransport::new();
    transport.queue_json(200, r#"{"api_semver":"25.0.0"}"#);

    let client = Client::new(Arc::clone(&transport), "http://busy.local").unwrap();

    assert_eq!(client.timeout(), None);

    client.system().version().await.unwrap();

    assert_eq!(Timeout::of(&transport.requests()[0]), None);
}

#[tokio::test]
async fn carries_the_configured_timeout_on_every_request() {
    let transport = StubTransport::new();
    transport.queue_json(200, r#"{"api_semver":"25.0.0"}"#);
    transport.queue_json(200, r#"{"state":"connected"}"#);

    let client = ClientBuilder::new("http://busy.local")
        .unwrap()
        .timeout(Duration::from_millis(250))
        .build(Arc::clone(&transport));

    assert_eq!(client.timeout(), Some(Duration::from_millis(250)));

    client.system().version().await.unwrap();
    client.wifi().status().await.unwrap();

    for request in transport.requests() {
        assert_eq!(Timeout::of(&request), Some(Duration::from_millis(250)));
    }
}

#[tokio::test]
async fn overrides_the_timeout_for_a_single_call() {
    let transport = StubTransport::new();
    transport.queue_json(200, r#"{"result":"OK"}"#);
    transport.queue_json(200, r#"{"result":"OK"}"#);
    transport.queue_json(200, r#"{"api_semver":"25.0.0"}"#);

    let client = ClientBuilder::new("http://busy.local")
        .unwrap()
        .timeout(Duration::from_secs(2))
        .build(Arc::clone(&transport));

    client
        .with_timeout(Duration::from_secs(600))
        .updater()
        .update(Bytes::from_static(b"tarball"))
        .await
        .unwrap();

    client.without_timeout().ble().enable().await.unwrap();
    client.system().version().await.unwrap();

    let requests = transport.requests();

    assert_eq!(Timeout::of(&requests[0]), Some(Duration::from_secs(600)));
    assert_eq!(Timeout::of(&requests[1]), None);
    assert_eq!(Timeout::of(&requests[2]), Some(Duration::from_secs(2)));
    assert_eq!(client.timeout(), Some(Duration::from_secs(2)));
}

#[tokio::test]
async fn surfaces_transport_failures_with_their_source() {
    let transport = StubTransport::new();
    let client = Client::new(Arc::clone(&transport), "http://busy.local").unwrap();

    let error = client.wifi().status().await.unwrap_err();

    let Error::Transport { ref source, .. } = error else {
        panic!("expected a transport error, got {error:?}");
    };

    assert_eq!(source.to_string(), "no response queued");
    assert_eq!(
        error.to_string(),
        "GET /api/wifi/status unable to reach device"
    );
}

#[tokio::test]
async fn mounts_the_api_under_the_configured_prefix() {
    let transport = StubTransport::new();

    let device = ClientBuilder::new("http://busy.local")
        .unwrap()
        .build(Arc::clone(&transport));
    let cloud = ClientBuilder::new("https://api.busy.app")
        .unwrap()
        .api_prefix(ApiPrefix::Cloud)
        .build(Arc::clone(&transport));
    let proxied = ClientBuilder::new("http://gateway.local/bars/one/")
        .unwrap()
        .api_prefix(ApiPrefix::Custom(PathPrefix::new("busy-bar/api").unwrap()))
        .build(Arc::clone(&transport));

    for client in [&device, &cloud, &proxied] {
        transport.queue_json(200, r#"{"api_semver":"25.0.0"}"#);
        client.system().version().await.unwrap();
    }

    let requests = transport.requests();
    let uris: Vec<String> = requests.iter().map(|r| r.uri().to_string()).collect();

    assert_eq!(
        uris,
        [
            "http://busy.local/api/version",
            "https://api.busy.app/busybar/version",
            "http://gateway.local/bars/one/busy-bar/api/version",
        ]
    );
    assert_eq!(device.api_prefix(), ApiPrefix::Device);
}

#[tokio::test]
async fn reports_the_resolved_path_when_a_call_fails() {
    let transport = StubTransport::new();
    let client = ClientBuilder::new("https://api.busy.app")
        .unwrap()
        .api_prefix(ApiPrefix::Cloud)
        .build(Arc::clone(&transport));

    let error = client.wifi().status().await.unwrap_err();

    assert_eq!(
        error.to_string(),
        "GET /busybar/wifi/status unable to reach device"
    );
}
