use super::action::Action;
use super::{ActionUuid, Instance};

use crate::OpenActionResult as Result;
use crate::inbound::{
	DialPressPayload, DialRotatePayload, DidReceiveSettingsPayload, GenericInstancePayload,
	TitleParametersDidChangePayload,
};

use async_trait::async_trait;

#[async_trait]
pub(super) trait ErasedAction: Send + Sync {
	fn uuid(&self) -> ActionUuid;

	async fn call_will_appear(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()>;
	async fn call_will_disappear(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()>;
	async fn call_key_down(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()>;
	async fn call_key_up(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()>;
	async fn call_dial_rotate(&self, instance: &Instance, event: DialRotatePayload) -> Result<()>;
	async fn call_dial_down(&self, instance: &Instance, event: DialPressPayload) -> Result<()>;
	async fn call_dial_up(&self, instance: &Instance, event: DialPressPayload) -> Result<()>;
	async fn call_did_receive_settings(&self, instance: &Instance, event: DidReceiveSettingsPayload) -> Result<()>;
	async fn call_title_parameters_did_change(
		&self,
		instance: &Instance,
		event: TitleParametersDidChangePayload,
	) -> Result<()>;
	async fn call_pi_did_appear(&self, instance: &Instance) -> Result<()>;
	async fn call_pi_did_disappear(&self, instance: &Instance) -> Result<()>;
}

fn deserialize_settings<A: Action>(json: serde_json::Value) -> A::Settings {
	match serde_json::from_value(json) {
		Ok(settings) => settings,
		Err(error) => {
			log::error!("Failed to deserialize settings for action {}: {}", A::UUID, error);
			Default::default()
		}
	}
}

pub(super) struct ActionWrapper<A: Action>(pub(super) A);

#[async_trait]
impl<A: Action> ErasedAction for ActionWrapper<A> {
	fn uuid(&self) -> ActionUuid {
		A::UUID
	}

	async fn call_will_appear(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0.will_appear(instance, &settings).await
	}

	async fn call_will_disappear(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0.will_disappear(instance, &settings).await
	}

	async fn call_key_down(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0.key_down(instance, &settings).await
	}

	async fn call_key_up(&self, instance: &Instance, event: GenericInstancePayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0.key_up(instance, &settings).await
	}

	async fn call_dial_rotate(&self, instance: &Instance, event: DialRotatePayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0
			.dial_rotate(instance, &settings, event.ticks, event.pressed)
			.await
	}

	async fn call_dial_down(&self, instance: &Instance, event: DialPressPayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0.dial_down(instance, &settings).await
	}

	async fn call_dial_up(&self, instance: &Instance, event: DialPressPayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0.dial_up(instance, &settings).await
	}

	async fn call_did_receive_settings(&self, instance: &Instance, event: DidReceiveSettingsPayload) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings);
		self.0.did_receive_settings(instance, &settings).await
	}

	async fn call_title_parameters_did_change(
		&self,
		instance: &Instance,
		event: TitleParametersDidChangePayload,
	) -> Result<()> {
		let settings = deserialize_settings::<A>(event.settings.clone());
		self.0.title_parameters_did_change(instance, &settings, &event).await
	}

	async fn call_pi_did_appear(&self, instance: &Instance) -> Result<()> {
		let settings = deserialize_settings::<A>(instance.settings_json.read().await.clone());
		self.0.property_inspector_did_appear(instance, &settings).await
	}

	async fn call_pi_did_disappear(&self, instance: &Instance) -> Result<()> {
		let settings = deserialize_settings::<A>(instance.settings_json.read().await.clone());
		self.0.property_inspector_did_disappear(instance, &settings).await
	}
}
