use super::{ContextAndPayloadEvent, ContextEvent, OutboundEventManager};

use crate::OpenActionResult as Result;

impl OutboundEventManager {
	pub async fn set_settings(&mut self, context: String, payload: serde_json::Value) -> Result<()> {
		self.send_event(ContextAndPayloadEvent {
			event: "setSettings",
			context,
			payload,
		})
		.await
	}

	pub async fn get_settings(&mut self, context: String) -> Result<()> {
		self.send_event(ContextEvent {
			event: "getSettings",
			context,
		})
		.await
	}

	pub async fn set_global_settings(&mut self, payload: serde_json::Value) -> Result<()> {
		self.send_event(ContextAndPayloadEvent {
			event: "setGlobalSettings",
			context: self.uuid.clone(),
			payload,
		})
		.await
	}

	pub async fn get_global_settings(&mut self) -> Result<()> {
		self.send_event(ContextEvent {
			event: "getGlobalSettings",
			context: self.uuid.clone(),
		})
		.await
	}
}
