mod devices;
mod misc;
mod settings;
mod states;

use crate::OpenActionResult as Result;

use futures_util::{SinkExt, stream::SplitSink};
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

type Sink =
	SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>;

/// A struct with methods for sending events to the OpenAction server
pub(crate) struct OutboundEventManager {
	sink: Sink,
}

impl OutboundEventManager {
	pub(crate) fn new(sink: Sink) -> Self {
		Self { sink }
	}

	pub async fn send_event(&mut self, event: impl Serialize) -> Result<()> {
		self.sink
			.send(Message::Text(serde_json::to_string(&event)?.into()))
			.await?;
		Ok(())
	}
}

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
