use super::GenericInstancePayload;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppearEvent {
	pub action: String,
	pub context: String,
	pub device: String,
	pub payload: GenericInstancePayload,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PropertyInspectorAppearEvent {
	pub action: String,
	pub context: String,
	pub device: String,
}
