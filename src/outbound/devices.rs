use super::{OutboundEventManager, PayloadEvent};

use crate::OpenActionResult as Result;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DeviceInfo {
	pub id: String,
	pub name: String,
	pub rows: u8,
	pub columns: u8,
	pub encoders: u8,
	pub r#type: u8,
}

#[derive(Serialize)]
pub struct PressPayload {
	pub device: String,
	pub position: u8,
}

#[derive(Serialize)]
pub struct TicksPayload {
	pub device: String,
	pub position: u8,
	pub ticks: i16,
}

impl OutboundEventManager {
	pub async fn register_device(
		&mut self,
		id: String,
		name: String,
		rows: u8,
		columns: u8,
		encoders: u8,
		r#type: u8,
	) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "registerDevice",
			payload: DeviceInfo {
				id,
				name,
				rows,
				columns,
				encoders,
				r#type,
			},
		})
		.await
	}

	pub async fn deregister_device(&mut self, id: String) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "deregisterDevice",
			payload: id,
		})
		.await
	}

	pub async fn rerender_images(&mut self, id: String) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "rerenderImages",
			payload: id,
		})
		.await
	}

	pub async fn key_down(&mut self, device: String, position: u8) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "keyDown",
			payload: PressPayload { device, position },
		})
		.await
	}

	pub async fn key_up(&mut self, device: String, position: u8) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "keyUp",
			payload: PressPayload { device, position },
		})
		.await
	}

	pub async fn encoder_change(&mut self, device: String, position: u8, ticks: i16) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "encoderChange",
			payload: TicksPayload {
				device,
				position,
				ticks,
			},
		})
		.await
	}

	pub async fn encoder_down(&mut self, device: String, position: u8) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "encoderDown",
			payload: PressPayload { device, position },
		})
		.await
	}

	pub async fn encoder_up(&mut self, device: String, position: u8) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "encoderUp",
			payload: PressPayload { device, position },
		})
		.await
	}
}
