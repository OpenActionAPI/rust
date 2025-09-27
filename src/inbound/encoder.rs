use super::{Coordinates, SettingsValue};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DialRotatePayload {
	pub settings: SettingsValue,
	#[allow(dead_code)]
	pub coordinates: Coordinates,
	pub ticks: i16,
	pub pressed: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DialRotateEvent {
	pub action: String,
	pub context: String,
	#[allow(dead_code)]
	pub device: String,
	pub payload: DialRotatePayload,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DialPressPayload {
	#[allow(dead_code)]
	pub controller: String,
	pub settings: SettingsValue,
	#[allow(dead_code)]
	pub coordinates: Coordinates,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DialPressEvent {
	pub action: String,
	pub context: String,
	#[allow(dead_code)]
	pub device: String,
	pub payload: DialPressPayload,
}
