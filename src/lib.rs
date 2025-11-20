mod inbound;
mod outbound;
mod runtime;

pub use crate::inbound::{Coordinates, TitleParametersDidChangePayload};
pub use crate::runtime::*;

/// Events that do not relate to a specific instance of an action
pub mod global_events {
	pub use super::inbound::{
		DeviceDidConnectEvent, DeviceDidDisconnectEvent, DidReceiveGlobalSettingsEvent, GlobalEventHandler,
		SetBrightnessEvent, SetImageEvent, SystemDidWakeUpEvent, set_global_event_handler,
	};
}

/// [`mod@async_trait`]
pub use async_trait::async_trait;

use futures_util::StreamExt;
use thiserror::Error;
use tokio_tungstenite::connect_async;

#[derive(Debug, Error)]
pub enum OpenActionError {
	#[error("WebSocket error: {0}")]
	WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

	#[error("serialization or deserialization error: {0}")]
	Serde(#[from] serde_json::Error),
}

pub type OpenActionResult<T> = Result<T, OpenActionError>;

/// Register the plugin and run the plugin event loop, blocking until disconnect
/// ```rust
/// use openaction::*;
///
/// #[tokio::main]
/// async fn main() -> OpenActionResult<()> {
///     // Initialize logger...
///     // Register actions...
///     run(std::env::args().collect()).await
/// }
/// ```
pub async fn run(args: Vec<String>) -> OpenActionResult<()> {
	let lookup = |flag: &str| -> String {
		let i = args
			.iter()
			.position(|x| x.to_lowercase().trim() == flag)
			.unwrap_or_else(|| panic!("missing CLI flag: {}", flag));
		args.get(i + 1)
			.cloned()
			.unwrap_or_else(|| panic!("missing CLI flag value for flag: {}", flag))
	};
	let port = lookup("-port");
	let uuid = lookup("-pluginuuid");
	let event = lookup("-registerevent");

	let socket = connect_async(format!("ws://localhost:{}", port)).await?.0;
	let (write, read) = socket.split();

	let mut outbound = outbound::OutboundEventManager::new(write);
	outbound.register(event, uuid).await?;
	runtime::set_outbound_manager(outbound).await;

	inbound::process_incoming_messages(read).await;

	Ok(())
}
