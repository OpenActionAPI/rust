use super::RUNTIME;

use crate::OpenActionResult as Result;

/// <https://openaction.amankhanna.me/5_serverbound/settings.html#getglobalsettings>
pub async fn get_global_settings() -> Result<()> {
	if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
		mgr.get_global_settings().await?;
	}
	Ok(())
}

/// <https://openaction.amankhanna.me/5_serverbound/settings.html#setglobalsettings>
pub async fn set_global_settings(value: impl serde::Serialize) -> Result<()> {
	let value = serde_json::to_value(value)?;
	if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
		mgr.set_global_settings(value).await?;
	}
	Ok(())
}

/// <https://openaction.amankhanna.me/5_serverbound/misc.html#openurl>
pub async fn open_url(url: String) -> Result<()> {
	if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
		mgr.open_url(url).await?;
	}
	Ok(())
}

/// <https://openaction.amankhanna.me/5_serverbound/misc.html#logmessage>
pub async fn log_message(message: String) -> Result<()> {
	if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
		mgr.log_message(message).await?;
	}
	Ok(())
}

/// Outbound events sent by plugins that add support for new devices
pub mod device_plugin {
	use super::{RUNTIME, Result};

	pub async fn register_device(
		id: String,
		name: String,
		rows: u8,
		columns: u8,
		encoders: u8,
		r#type: u8,
	) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.register_device(id, name, rows, columns, encoders, r#type).await?;
		}
		Ok(())
	}

	pub async fn unregister_device(id: String) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.deregister_device(id).await?;
		}
		Ok(())
	}

	pub async fn rerender_images(id: String) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.rerender_images(id).await?;
		}
		Ok(())
	}

	pub async fn key_down(device: String, position: u8) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.key_down(device, position).await?;
		}
		Ok(())
	}

	pub async fn key_up(device: String, position: u8) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.key_up(device, position).await?;
		}
		Ok(())
	}

	pub async fn encoder_change(device: String, position: u8, ticks: i16) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.encoder_change(device, position, ticks).await?;
		}
		Ok(())
	}

	pub async fn encoder_down(device: String, position: u8) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.encoder_down(device, position).await?;
		}
		Ok(())
	}

	pub async fn encoder_up(device: String, position: u8) -> Result<()> {
		if let Some(mgr) = RUNTIME.outbound.lock().await.as_mut() {
			mgr.encoder_up(device, position).await?;
		}
		Ok(())
	}
}
