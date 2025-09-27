use super::{Coordinates, GenericInstancePayload, SettingsValue};

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
	#[allow(dead_code)]
	pub device: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TitleParametersDidChangeEvent {
	pub action: String,
	pub context: String,
	#[allow(dead_code)]
	pub device: String,
	pub payload: TitleParametersDidChangePayload,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TitleParametersDidChangePayload {
	pub settings: SettingsValue,
	pub coordinates: Coordinates,
	pub state: u16,
	pub title: String,
	pub titleParameters: TitleParameters,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct TitleParameters {
	pub fontFamily: String,
	pub fontSize: u16,
	pub fontStyle: String,
	pub fontUnderline: bool,
	pub showTitle: bool,
	pub titleAlignment: String,
	pub titleColor: String,
}
