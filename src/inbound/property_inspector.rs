use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PropertyInspectorAppearEvent {
	pub action: String,
	pub context: String,
	#[allow(dead_code)]
	pub device: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SendToPluginEvent {
	pub action: String,
	pub context: String,
	pub payload: serde_json::Value,
}
