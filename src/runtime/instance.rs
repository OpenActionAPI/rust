use super::InstanceId;

use crate::OpenActionResult as Result;
use crate::inbound::Coordinates;
use crate::outbound::OutboundEventManager;

use serde_json::Value;
use tokio::sync::RwLock;

/// An instance of an action bound to the device surface
pub struct Instance {
	/// The UUID of the action this is an instance of
	pub action_uuid: String,
	/// An ID that uniquely identifies this instance
	pub instance_id: InstanceId,
	/// The device this instance is bound to
	pub device_id: String,
	/// The controller this instance is bound to
	pub controller: String,
	/// Coordinates on the device surface where this instance is bound
	pub coordinates: Coordinates,
	/// Whether or not this instance is part of a Multi Action
	pub is_in_multi_action: bool,
	/// Index of the currently active state within the states defined in the plugin manifest
	pub current_state_index: u16,
	pub(crate) settings_json: RwLock<Value>,
}

impl Instance {
	async fn outbound(&self) -> tokio::sync::MutexGuard<'_, Option<OutboundEventManager>> {
		super::RUNTIME.outbound.lock().await
	}

	/// <https://openaction.amankhanna.me/5_serverbound/states.html#settitle>
	pub async fn set_title(&self, title: impl Into<String>) -> Result<()> {
		if let Some(mgr) = self.outbound().await.as_mut() {
			mgr.set_title(self.instance_id.clone(), Some(title.into()), None)
				.await?;
		}
		Ok(())
	}

	/// <https://openaction.amankhanna.me/5_serverbound/states.html#setimage>
	pub async fn set_image(&self, image: impl Into<String>) -> Result<()> {
		if let Some(mgr) = self.outbound().await.as_mut() {
			mgr.set_image(self.instance_id.clone(), Some(image.into()), None)
				.await?;
		}
		Ok(())
	}

	/// <https://openaction.amankhanna.me/5_serverbound/states.html#setstate>
	pub async fn set_state(&self, state: u16) -> Result<()> {
		if let Some(mgr) = self.outbound().await.as_mut() {
			mgr.set_state(self.instance_id.clone(), state).await?;
		}
		Ok(())
	}

	/// <https://openaction.amankhanna.me/5_serverbound/misc.html#showalert>
	pub async fn show_alert(&self) -> Result<()> {
		if let Some(mgr) = self.outbound().await.as_mut() {
			mgr.show_alert(self.instance_id.clone()).await?;
		}
		Ok(())
	}

	/// <https://openaction.amankhanna.me/5_serverbound/misc.html#showok>
	pub async fn show_ok(&self) -> Result<()> {
		if let Some(mgr) = self.outbound().await.as_mut() {
			mgr.show_ok(self.instance_id.clone()).await?;
		}
		Ok(())
	}

	/// <https://openaction.amankhanna.me/5_serverbound/settings.html#setsettings>
	pub async fn set_settings(&self, value: &impl serde::Serialize) -> Result<()> {
		let value = serde_json::to_value(value)?;
		if let Some(rec) = super::RUNTIME.instances.get(&self.instance_id) {
			*rec.settings_json.write().await = value.clone();
			if let Some(mgr) = self.outbound().await.as_mut() {
				mgr.set_settings(self.instance_id.clone(), value).await?;
			}
		}
		Ok(())
	}

	/// <https://openaction.amankhanna.me/5_serverbound/settings.html#getsettings>
	pub async fn get_settings(&self) -> Result<()> {
		if let Some(mgr) = self.outbound().await.as_mut() {
			mgr.get_settings(self.instance_id.clone()).await?;
		}
		Ok(())
	}

	/// <https://openaction.amankhanna.me/5_serverbound/property_inspector.html#sendtopropertyinspector>
	pub async fn send_to_property_inspector(&self, value: impl serde::Serialize) -> Result<()> {
		let value = serde_json::to_value(value)?;
		if let Some(mgr) = self.outbound().await.as_mut() {
			mgr.send_to_property_inspector(self.instance_id.clone(), value).await?;
		}
		Ok(())
	}
}
