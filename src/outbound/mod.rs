mod devices;
mod misc;
mod settings;
mod states;

use futures_util::{stream::SplitSink, SinkExt};
use serde::Serialize;
use std::sync::LazyLock;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Error, Message};

type Sink =
	SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>;

/// A struct with methods for sending events to the OpenAction server.
pub struct OutboundEventManager {
	sink: Sink,
}

impl OutboundEventManager {
	pub(crate) fn new(sink: Sink) -> Self {
		Self { sink }
	}

	pub async fn send_event(&mut self, event: impl Serialize) -> Result<(), Error> {
		self.sink
			.send(Message::Text(serde_json::to_string(&event).unwrap().into()))
			.await
	}
}

/// The outbound event manager available for access outside of event handlers.
pub static OUTBOUND_EVENT_MANAGER: LazyLock<Mutex<Option<OutboundEventManager>>> = LazyLock::new(|| Mutex::new(None));

#[derive(Serialize)]
struct SimpleEvent {
	event: &'static str,
}

#[derive(Serialize)]
struct ContextEvent {
	event: &'static str,
	context: String,
}

#[derive(Serialize)]
struct PayloadEvent<T> {
	event: &'static str,
	payload: T,
}

#[derive(Serialize)]
struct ContextAndPayloadEvent<T> {
	event: &'static str,
	context: String,
	payload: T,
}
