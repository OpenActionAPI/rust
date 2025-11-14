use super::instance::Instance;

use crate::OpenActionResult as Result;

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

#[async_trait]
/// Event handler trait for events that relate to a specific instance of an action
pub trait Action: Send + Sync + 'static {
	/// Action UUID as defined in the plugin manifest
	const UUID: &'static str;

	/// Settings type for this action
	type Settings: Serialize + DeserializeOwned + Default + Send + Sync + 'static;

	/// <https://openaction.amankhanna.me/4_clientbound/will_appear.html#willappear>
	async fn will_appear(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/will_appear.html#willdisappear>
	async fn will_disappear(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/keypad.html#keydown>
	async fn key_down(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/keypad.html#keyup>
	async fn key_up(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/encoder.html#dialrotate>
	async fn dial_rotate(
		&self,
		_instance: &Instance,
		_settings: &Self::Settings,
		_ticks: i16,
		_pressed: bool,
	) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/encoder.html#dialdown>
	async fn dial_down(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/encoder.html#dialup>
	async fn dial_up(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/settings.html#didreceivesettings>
	async fn did_receive_settings(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/states.html#titleparametersdidchange>
	async fn title_parameters_did_change(
		&self,
		_instance: &Instance,
		_settings: &Self::Settings,
		_event: &crate::inbound::TitleParametersDidChangePayload,
	) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/property_inspector.html#propertyinspectordidappear>
	async fn property_inspector_did_appear(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	/// <https://openaction.amankhanna.me/4_clientbound/property_inspector.html#propertyinspectordiddisappear>
	async fn property_inspector_did_disappear(&self, _instance: &Instance, _settings: &Self::Settings) -> Result<()> {
		Ok(())
	}

	async fn send_to_plugin(
		&self,
		_instance: &Instance,
		_settings: &Self::Settings,
		_payload: &serde_json::Value,
	) -> Result<()> {
		Ok(())
	}
}
