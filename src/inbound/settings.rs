use super::{GenericInstancePayload, SettingsValue};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DidReceiveSettingsEvent {
	pub action: String,
	pub context: String,
	#[allow(dead_code)]
	pub device: String,
	pub payload: GenericInstancePayload,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DidReceiveGlobalSettingsPayload {
	pub settings: SettingsValue,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DidReceiveGlobalSettingsEvent {
	pub payload: DidReceiveGlobalSettingsPayload,
}
