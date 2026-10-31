use super::{OutboundEventManager, PayloadEvent};

use crate::OpenActionResult as Result;

use serde::Serialize;

/// How the rotary encoders of a device are laid out in their row
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EncoderAlignment {
	/// Spread the encoders across the whole row
	#[default]
	Between,
	/// Line the encoders up at the start of the row
	Start,
	/// Line the encoders up in the middle of the row
	Center,
	/// Line the encoders up at the end of the row
	End,
}

/// A physical control that occupies a slot in a device's grid
#[derive(Clone, Copy, Debug, Serialize)]
pub struct DeviceSlot {
	/// The position of the slot in the device's grid
	pub position: u8,
	/// Whether the slot is round, like a dial
	#[serde(skip_serializing_if = "is_false")]
	pub round: bool,
	/// The index of the rotary encoder occupying the slot, if any
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encoder: Option<u8>,
}

fn is_false(value: &bool) -> bool {
	!*value
}

/// How a device presents itself to the user
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeviceDecoration {
	/// An image (data URL or http(s) URL) drawn behind the device's buttons
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background: Option<String>,
	/// How the rotary encoders are laid out in their row
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encoder_alignment: Option<EncoderAlignment>,
	/// The physical controls that occupy the device's grid
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slots: Option<Vec<DeviceSlot>>,
}

/// How a plugin registers a device it adds support for
#[derive(Clone, Debug, Default)]
pub struct DeviceDescriptor {
	/// The id of the device
	pub id: String,
	/// The name shown to the user
	pub name: String,
	/// The number of rows of buttons
	pub rows: u8,
	/// The number of columns of buttons
	pub columns: u8,
	/// The number of rotary encoders
	pub encoders: u8,
	/// The type of the device
	pub r#type: u8,
	/// How the device presents itself to the user
	pub decoration: DeviceDecoration,
}

#[derive(Clone, Serialize)]
pub struct DeviceInfo {
	pub id: String,
	pub name: String,
	pub rows: u8,
	pub columns: u8,
	pub encoders: u8,
	pub r#type: u8,
	#[serde(flatten)]
	pub decoration: DeviceDecoration,
}

#[derive(Clone, Serialize)]
pub struct DeviceBackgroundPayload {
	pub device: String,
	pub background: Option<String>,
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

#[derive(Serialize)]
pub struct TouchscreenPressPayload {
	pub device: String,
	pub position: u8,
	pub x: u16,
	pub y: u16,
	pub hold: bool,
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
		self.register_device_with(DeviceDescriptor {
			id,
			name,
			rows,
			columns,
			encoders,
			r#type,
			decoration: DeviceDecoration::default(),
		})
		.await
	}

	pub async fn register_device_with(&mut self, descriptor: DeviceDescriptor) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "registerDevice",
			payload: DeviceInfo {
				id: descriptor.id,
				name: descriptor.name,
				rows: descriptor.rows,
				columns: descriptor.columns,
				encoders: descriptor.encoders,
				r#type: descriptor.r#type,
				decoration: descriptor.decoration,
			},
		})
		.await
	}

	pub async fn set_device_background(&mut self, device: String, background: Option<String>) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "setDeviceBackground",
			payload: DeviceBackgroundPayload { device, background },
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

	pub async fn touchscreen_press(&mut self, device: String, position: u8, x: u16, y: u16, hold: bool) -> Result<()> {
		self.send_event(PayloadEvent {
			event: "touchscreenPress",
			payload: TouchscreenPressPayload {
				device,
				position,
				x,
				y,
				hold,
			},
		})
		.await
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn device_info(decoration: DeviceDecoration) -> DeviceInfo {
		DeviceInfo {
			id: "ex-0123456789ab".to_owned(),
			name: "Example Controller".to_owned(),
			rows: 4,
			columns: 5,
			encoders: 1,
			r#type: 0,
			decoration,
		}
	}

	#[test]
	fn a_device_without_a_decoration_only_sends_its_geometry() {
		let payload = serde_json::to_value(device_info(DeviceDecoration::default())).unwrap();
		let object = payload.as_object().unwrap();

		assert_eq!(object["id"], "ex-0123456789ab");
		assert_eq!(object["rows"], 4);
		assert_eq!(object["columns"], 5);
		assert_eq!(object["encoders"], 1);
		assert_eq!(object["type"], 0);
		assert!(!object.contains_key("background"));
		assert!(!object.contains_key("encoder_alignment"));
		assert!(!object.contains_key("slots"));
	}

	#[test]
	fn a_decorated_device_sends_its_background_and_slots() {
		let payload = serde_json::to_value(device_info(DeviceDecoration {
			background: Some("data:image/png;base64,AAAA".to_owned()),
			encoder_alignment: Some(EncoderAlignment::Center),
			slots: Some(vec![
				DeviceSlot {
					position: 12,
					round: true,
					encoder: Some(0),
				},
				DeviceSlot {
					position: 1,
					round: false,
					encoder: None,
				},
			]),
		}))
		.unwrap();
		let object = payload.as_object().unwrap();

		assert_eq!(object["background"], "data:image/png;base64,AAAA");
		assert_eq!(object["encoder_alignment"], "center");
		assert_eq!(object["slots"][0]["position"], 12);
		assert_eq!(object["slots"][0]["round"], true);
		assert_eq!(object["slots"][0]["encoder"], 0);
		// Slots that are plain buttons only send their position.
		assert_eq!(object["slots"][1], serde_json::json!({ "position": 1 }));
	}

	#[test]
	fn a_background_names_the_device_it_belongs_to() {
		let payload = serde_json::to_value(DeviceBackgroundPayload {
			device: "ex-0123456789ab".to_owned(),
			background: Some("data:image/png;base64,AAAA".to_owned()),
		})
		.unwrap();
		assert_eq!(
			payload,
			serde_json::json!({ "device": "ex-0123456789ab", "background": "data:image/png;base64,AAAA" })
		);
	}

	#[test]
	fn a_background_can_be_cleared() {
		let payload = serde_json::to_value(DeviceBackgroundPayload {
			device: "ex-0123456789ab".to_owned(),
			background: None,
		})
		.unwrap();

		assert_eq!(
			payload,
			serde_json::json!({ "device": "ex-0123456789ab", "background": null })
		);
	}

	#[test]
	fn the_alignment_is_sent_in_lowercase() {
		assert_eq!(serde_json::to_value(EncoderAlignment::Between).unwrap(), "between");
		assert_eq!(serde_json::to_value(EncoderAlignment::Start).unwrap(), "start");
		assert_eq!(serde_json::to_value(EncoderAlignment::Center).unwrap(), "center");
		assert_eq!(serde_json::to_value(EncoderAlignment::End).unwrap(), "end");
	}

	#[test]
	fn a_slot_only_sends_what_it_uses() {
		let round = serde_json::to_value(DeviceSlot {
			position: 12,
			round: true,
			encoder: Some(0),
		})
		.unwrap();
		assert_eq!(
			round,
			serde_json::json!({ "position": 12, "round": true, "encoder": 0 })
		);

		let plain = serde_json::to_value(DeviceSlot {
			position: 1,
			round: false,
			encoder: None,
		})
		.unwrap();
		assert_eq!(plain, serde_json::json!({ "position": 1 }));
	}
}
