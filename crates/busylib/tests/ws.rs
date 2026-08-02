#![cfg(feature = "ws")]

use std::net::SocketAddr;

use busylib::api::StatusStream;
use busylib::proto::bsb_frame::{Encoding, Frame, PixelFormat, Screen};
use busylib::proto::bsb_state::state_update::State as Update;
use busylib::proto::bsb_state::{State, StateUpdate};
use busylib::{ClientBuilder, TungsteniteWsTransport};
use futures_util::{SinkExt as _, StreamExt as _};
use prost::Message as _;
use tokio::io::AsyncWriteExt as _;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;

fn frame_state() -> State {
    State {
        timestamp: 1785692728867,
        updates: vec![StateUpdate {
            state: Some(Update::Frame(Frame {
                screen: Screen::Front as i32,
                width: 72,
                height: 16,
                encoding: Encoding::RunLength as i32,
                pixel_format: PixelFormat::Rgb888 as i32,
                data: vec![0x7f, 0x00, 0x00, 0x00],
            })),
        }],
        error: None,
    }
}

fn encoded(state: &State) -> Message {
    Message::binary(state.encode_to_vec())
}

async fn listener() -> (SocketAddr, TcpListener) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    (address, listener)
}

fn serve(listener: TcpListener, messages: Vec<Message>) -> JoinHandle<String> {
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = tokio_tungstenite::accept_async(stream).await.unwrap();

        let control = match socket.next().await.unwrap().unwrap() {
            Message::Text(text) => text.as_str().to_owned(),
            other => panic!("expected the control message, got {other:?}"),
        };

        for message in messages {
            socket.send(message).await.unwrap();
        }

        socket.close(None).await.unwrap();

        control
    })
}

fn reject(listener: TcpListener) -> JoinHandle<()> {
    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        stream
            .write_all(b"HTTP/1.1 400 Bad Request\r\ncontent-length: 0\r\n\r\n")
            .await
            .unwrap();
        stream.flush().await.unwrap();
    })
}

async fn connect(address: SocketAddr) -> StatusStream {
    let client = ClientBuilder::new(format!("http://{address}"))
        .unwrap()
        .build_reqwest();

    client
        .streaming()
        .status_ws(&TungsteniteWsTransport::new())
        .await
        .unwrap()
}

#[tokio::test]
async fn enables_the_stream_on_connect() {
    let (address, listener) = listener().await;
    let server = serve(listener, vec![]);

    let mut stream = connect(address).await;
    while stream.next().await.is_some() {}

    assert_eq!(server.await.unwrap(), r#"{"enable":true}"#);
}

#[tokio::test]
async fn reads_a_state_message() {
    let (address, listener) = listener().await;
    let server = serve(listener, vec![encoded(&frame_state())]);

    let mut stream = connect(address).await;
    let state = stream.next().await.unwrap().unwrap();

    assert_eq!(state.timestamp, 1785692728867);
    assert_eq!(state.updates.len(), 1);

    let Some(Update::Frame(frame)) = &state.updates[0].state else {
        panic!("expected a frame update, got {:?}", state.updates[0]);
    };

    assert_eq!((frame.width, frame.height), (72, 16));
    assert_eq!(frame.screen(), Screen::Front);
    assert_eq!(frame.encoding(), Encoding::RunLength);
    assert_eq!(frame.pixel_format(), PixelFormat::Rgb888);

    server.await.unwrap();
}

#[tokio::test]
async fn reads_a_heartbeat_without_updates() {
    let state = State {
        timestamp: 1785692728867,
        updates: vec![],
        error: None,
    };
    let (address, listener) = listener().await;
    let server = serve(listener, vec![encoded(&state)]);

    let mut stream = connect(address).await;
    let state = stream.next().await.unwrap().unwrap();

    assert_eq!(state.timestamp, 1785692728867);
    assert!(state.updates.is_empty());

    server.await.unwrap();
}

#[tokio::test]
async fn skips_ping_and_ends_on_close() {
    let (address, listener) = listener().await;
    let server = serve(
        listener,
        vec![Message::Ping(Default::default()), encoded(&frame_state())],
    );

    let mut stream = connect(address).await;

    assert!(stream.next().await.unwrap().is_ok());
    assert!(stream.next().await.is_none());

    server.await.unwrap();
}

#[tokio::test]
async fn reports_a_payload_which_is_not_protobuf() {
    let (address, listener) = listener().await;
    let server = serve(listener, vec![Message::binary(vec![0xff, 0xff, 0xff])]);

    let mut stream = connect(address).await;
    let error = stream.next().await.unwrap().unwrap_err();

    assert!(error.to_string().contains("not a valid protobuf State"));
    assert!(error.to_string().contains("/api/status/ws"));

    server.await.unwrap();
}

#[tokio::test]
async fn reports_a_text_message_as_unexpected() {
    let (address, listener) = listener().await;
    let server = serve(listener, vec![Message::text("not protobuf")]);

    let mut stream = connect(address).await;
    let error = stream.next().await.unwrap().unwrap_err();

    assert!(error.to_string().contains("sent a text message"));
    assert!(error.to_string().contains("not protobuf"));

    server.await.unwrap();
}

#[tokio::test]
async fn surfaces_the_status_of_a_rejected_handshake() {
    let (address, listener) = listener().await;
    let server = reject(listener);

    let client = ClientBuilder::new(format!("http://{address}"))
        .unwrap()
        .build_reqwest();

    let error = client
        .streaming()
        .status_ws(&TungsteniteWsTransport::new())
        .await
        .unwrap_err();

    assert_eq!(error.status(), Some(http::StatusCode::BAD_REQUEST));
    assert!(error.to_string().contains("/api/status/ws"));

    server.await.unwrap();
}
