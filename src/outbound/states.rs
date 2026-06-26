use super::{ContextAndPayloadEvent, ContextEvent, OutboundEventManager};

use crate::OpenActionResult as Result;

use serde::Serialize;

#[derive(Serialize)]
struct SetTitlePayload {
	title: Option<String>,
	state: Option<u16>,
}

#[derive(Serialize)]
struct SetImagePayload {
	image: Option<String>,
	state: Option<u16>,
}

#[derive(Serialize)]
struct SetStatePayload {
	state: u16,
}

#[derive(Serialize)]
struct SetFeedbackLayoutPayload {
	layout: String,
}

impl OutboundEventManager {
	pub async fn set_title(&mut self, context: String, title: Option<String>, state: Option<u16>) -> Result<()> {
		self.send_event(ContextAndPayloadEvent {
			event: "setTitle",
			context,
			payload: SetTitlePayload { title, state },
		})
		.await
	}

	pub async fn set_image(&mut self, context: String, image: Option<String>, state: Option<u16>) -> Result<()> {
		self.send_event(ContextAndPayloadEvent {
			event: "setImage",
			context,
			payload: SetImagePayload { image, state },
		})
		.await
	}

	pub async fn set_state(&mut self, context: String, state: u16) -> Result<()> {
		self.send_event(ContextAndPayloadEvent {
			event: "setState",
			context,
			payload: SetStatePayload { state },
		})
		.await
	}

	pub async fn set_feedback(&mut self, context: String, feedback: &impl serde::Serialize) -> Result<()> {
		self.send_event(ContextAndPayloadEvent {
			event: "setFeedback",
			context,
			payload: feedback,
		})
		.await
	}

	pub async fn set_feedback_layout(&mut self, context: String, layout: String) -> Result<()> {
		self.send_event(ContextAndPayloadEvent {
			event: "setFeedbackLayout",
			context,
			payload: SetFeedbackLayoutPayload { layout },
		})
		.await
	}

	pub async fn show_alert(&mut self, context: String) -> Result<()> {
		self.send_event(ContextEvent {
			event: "showAlert",
			context,
		})
		.await
	}

	pub async fn show_ok(&mut self, context: String) -> Result<()> {
		self.send_event(ContextEvent {
			event: "showOk",
			context,
		})
		.await
	}
}
