use super::resolve;

use crate::OpenActionResult as Result;
use crate::inbound::{
	DialPressEvent, DialRotateEvent, DidReceiveSettingsEvent, KeyEvent, PropertyInspectorAppearEvent,
	TitleParametersDidChangeEvent,
};

async fn update_settings(instance: &super::instance::Instance, settings: &serde_json::Value) {
	*instance.settings_json.write().await = settings.clone();
}

pub(crate) async fn handle_key_down(event: KeyEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		update_settings(&instance, &event.payload.settings).await;
		action.call_key_down(&instance, event.payload).await?;
	}
	Ok(())
}

pub(crate) async fn handle_key_up(event: KeyEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		update_settings(&instance, &event.payload.settings).await;
		action.call_key_up(&instance, event.payload).await?;
	}
	Ok(())
}

pub(crate) async fn handle_dial_rotate(event: DialRotateEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		update_settings(&instance, &event.payload.settings).await;
		action.call_dial_rotate(&instance, event.payload).await?;
	}
	Ok(())
}

pub(crate) async fn handle_dial_down(event: DialPressEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		update_settings(&instance, &event.payload.settings).await;
		action.call_dial_down(&instance, event.payload).await?;
	}
	Ok(())
}

pub(crate) async fn handle_dial_up(event: DialPressEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		update_settings(&instance, &event.payload.settings).await;
		action.call_dial_up(&instance, event.payload).await?;
	}
	Ok(())
}

pub(crate) async fn handle_did_receive_settings(event: DidReceiveSettingsEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		update_settings(&instance, &event.payload.settings).await;
		action.call_did_receive_settings(&instance, event.payload).await?;
	}
	Ok(())
}

pub(crate) async fn handle_title_parameters_did_change(event: TitleParametersDidChangeEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		update_settings(&instance, &event.payload.settings).await;
		action
			.call_title_parameters_did_change(&instance, event.payload)
			.await?;
	}
	Ok(())
}

pub(crate) async fn handle_property_inspector_did_appear(event: PropertyInspectorAppearEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		action.call_pi_did_appear(&instance).await?;
	}
	Ok(())
}

pub(crate) async fn handle_property_inspector_did_disappear(event: PropertyInspectorAppearEvent) -> Result<()> {
	if let Some((action, instance)) = resolve(&event.action, &event.context).await? {
		action.call_pi_did_disappear(&instance).await?;
	}
	Ok(())
}
