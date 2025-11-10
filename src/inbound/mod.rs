mod devices;
mod encoder;
mod keypad;
mod misc;
mod settings;
mod will_appear;

pub use devices::*;
pub use encoder::*;
pub use keypad::*;
pub use misc::*;
pub use settings::*;
pub use will_appear::*;

use crate::{OpenActionResult as Result, runtime::inbound as runtime};

use std::sync::OnceLock;

use async_trait::async_trait;
use futures_util::{StreamExt, stream::SplitStream};
use serde::Deserialize;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Message};

type SettingsValue = serde_json::Value;

/// The coordinates of an action instance on the device surface
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct Coordinates {
	pub row: u8,
	pub column: u8,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GenericInstancePayload {
	pub settings: SettingsValue,
	pub coordinates: Option<Coordinates>,
	pub controller: String,
	#[serde(default)]
	pub state: u16,
	pub is_in_multi_action: bool,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
enum InboundEventType {
	/* Global events */
	SetImage(SetImageEvent),
	SetBrightness(SetBrightnessEvent),
	DidReceiveGlobalSettings(DidReceiveGlobalSettingsEvent),
	DeviceDidConnect(DeviceDidConnectEvent),
	DeviceDidDisconnect(DeviceDidDisconnectEvent),
	SystemDidWakeUp(SystemDidWakeUpEvent),
	/* Action events */
	KeyDown(KeyEvent),
	KeyUp(KeyEvent),
	DialDown(DialPressEvent),
	DialUp(DialPressEvent),
	DialRotate(DialRotateEvent),
	DidReceiveSettings(DidReceiveSettingsEvent),
	WillAppear(AppearEvent),
	WillDisappear(AppearEvent),
	PropertyInspectorDidAppear(PropertyInspectorAppearEvent),
	PropertyInspectorDidDisappear(PropertyInspectorAppearEvent),
	TitleParametersDidChange(TitleParametersDidChangeEvent),
}

#[async_trait]
/// Event handler trait for events that do not relate to a specific instance of an action
pub trait GlobalEventHandler: Send + Sync {
	async fn plugin_ready(&self) -> Result<()> {
		Ok(())
	}

	async fn device_plugin_set_image(&self, _event: SetImageEvent) -> Result<()> {
		Ok(())
	}

	async fn device_plugin_set_brightness(&self, _event: SetBrightnessEvent) -> Result<()> {
		Ok(())
	}

	async fn did_receive_global_settings(&self, _event: DidReceiveGlobalSettingsEvent) -> Result<()> {
		Ok(())
	}

	async fn device_did_connect(&self, _event: DeviceDidConnectEvent) -> Result<()> {
		Ok(())
	}

	async fn device_did_disconnect(&self, _event: DeviceDidDisconnectEvent) -> Result<()> {
		Ok(())
	}

	async fn system_did_wake_up(&self, _event: SystemDidWakeUpEvent) -> Result<()> {
		Ok(())
	}
}

static GLOBAL_EVENT_HANDLER: OnceLock<&'static dyn GlobalEventHandler> = OnceLock::new();

/// Register the handler for global events (does nothing if already set)
pub fn set_global_event_handler(handler: &'static dyn GlobalEventHandler) {
	let _ = GLOBAL_EVENT_HANDLER.set(handler);
}

pub(crate) async fn process_incoming_messages(
	mut stream: SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
) {
	if let Some(handler) = GLOBAL_EVENT_HANDLER.get()
		&& let Err(error) = handler.plugin_ready().await
	{
		log::error!("Failed to run plugin ready handler: {}", error);
	}

	while let Some(message) = stream.next().await {
		let Ok(data) = message else {
			continue;
		};

		if let Message::Text(text) = data {
			let decoded: InboundEventType = match serde_json::from_str(&text) {
				Ok(event) => event,
				Err(_) => {
					log::warn!("Unknown event received: {}", text);
					continue;
				}
			};

			if let Err(error) = match decoded {
				InboundEventType::SetImage(event) => {
					if let Some(h) = GLOBAL_EVENT_HANDLER.get() {
						h.device_plugin_set_image(event).await
					} else {
						Ok(())
					}
				}
				InboundEventType::SetBrightness(event) => {
					if let Some(h) = GLOBAL_EVENT_HANDLER.get() {
						h.device_plugin_set_brightness(event).await
					} else {
						Ok(())
					}
				}
				InboundEventType::DidReceiveGlobalSettings(event) => {
					if let Some(h) = GLOBAL_EVENT_HANDLER.get() {
						h.did_receive_global_settings(event).await
					} else {
						Ok(())
					}
				}
				InboundEventType::DeviceDidConnect(event) => {
					if let Some(h) = GLOBAL_EVENT_HANDLER.get() {
						h.device_did_connect(event).await
					} else {
						Ok(())
					}
				}
				InboundEventType::DeviceDidDisconnect(event) => {
					if let Some(h) = GLOBAL_EVENT_HANDLER.get() {
						h.device_did_disconnect(event).await
					} else {
						Ok(())
					}
				}
				InboundEventType::SystemDidWakeUp(event) => {
					if let Some(h) = GLOBAL_EVENT_HANDLER.get() {
						h.system_did_wake_up(event).await
					} else {
						Ok(())
					}
				}
				/* Instance events */
				InboundEventType::KeyDown(event) => runtime::handle_key_down(event).await,
				InboundEventType::KeyUp(event) => runtime::handle_key_up(event).await,
				InboundEventType::DialDown(event) => runtime::handle_dial_down(event).await,
				InboundEventType::DialUp(event) => runtime::handle_dial_up(event).await,
				InboundEventType::DialRotate(event) => runtime::handle_dial_rotate(event).await,
				InboundEventType::DidReceiveSettings(event) => runtime::handle_did_receive_settings(event).await,
				InboundEventType::WillAppear(event) => crate::runtime::handle_will_appear(event).await,
				InboundEventType::WillDisappear(event) => crate::runtime::handle_will_disappear(event).await,
				InboundEventType::PropertyInspectorDidAppear(event) => {
					runtime::handle_property_inspector_did_appear(event).await
				}
				InboundEventType::PropertyInspectorDidDisappear(event) => {
					runtime::handle_property_inspector_did_disappear(event).await
				}
				InboundEventType::TitleParametersDidChange(event) => {
					runtime::handle_title_parameters_did_change(event).await
				}
			} {
				log::error!("Failed to process inbound event: {}", error)
			}
		}
	}
}
