mod inbound;
mod outbound;

use tokio_tungstenite::connect_async;

use futures_util::StreamExt;

pub use inbound::*;
pub use outbound::{OutboundEventManager, OUTBOUND_EVENT_MANAGER};

pub type SettingsValue = serde_json::Value;

struct CliArgs {
	port: String,
	uuid: String,
	event: String,
}

/// Initialise the plugin and register it with the OpenAction server.
pub async fn init_plugin(
	global_event_handler: impl inbound::GlobalEventHandler,
	action_event_handler: impl inbound::ActionEventHandler,
) -> Result<(), anyhow::Error> {
	let args: Vec<_> = std::env::args().collect();
	let args = CliArgs {
		port: args[args.iter().position(|x| x.to_lowercase().trim() == "-port").unwrap() + 1].clone(),
		uuid: args[args
			.iter()
			.position(|x| x.to_lowercase().trim() == "-pluginuuid")
			.unwrap() + 1]
			.clone(),
		event: args[args
			.iter()
			.position(|x| x.to_lowercase().trim() == "-registerevent")
			.unwrap() + 1]
			.clone(),
	};

	let socket = connect_async(format!("ws://localhost:{}", args.port)).await?.0;
	let (write, read) = socket.split();

	let mut outbound = OutboundEventManager::new(write);
	outbound.register(args.event, args.uuid).await?;
	*outbound::OUTBOUND_EVENT_MANAGER.lock().await = Some(outbound);

	inbound::process_incoming_messages(read, global_event_handler, action_event_handler).await;

	Ok(())
}
