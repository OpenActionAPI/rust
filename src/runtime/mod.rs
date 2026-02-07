mod action;
pub(crate) mod inbound;
mod instance;
mod outbound;
mod wrapper;

pub use action::Action;
pub use instance::Instance;
pub use outbound::*;
use wrapper::{ActionWrapper, ErasedAction};

use crate::OpenActionResult as Result;
use crate::inbound::AppearEvent;
use crate::outbound::OutboundEventManager;

use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use dashmap::{DashMap, DashSet};
use tokio::sync::{Mutex, RwLock};

/// UUID of an action as defined in the plugin manifest
pub type ActionUuid = &'static str;
/// Value uniquely identifying an instance of an action
pub type InstanceId = String;

#[derive(Default)]
struct Runtime {
	actions: Mutex<HashMap<ActionUuid, Arc<dyn ErasedAction>>>,
	instances: DashMap<InstanceId, Arc<Instance>>,
	visible: DashMap<ActionUuid, DashSet<InstanceId>>,
	outbound: Mutex<Option<OutboundEventManager>>,
}

static RUNTIME: LazyLock<Runtime> = LazyLock::new(Runtime::default);

pub(crate) async fn set_outbound_manager(mgr: OutboundEventManager) {
	let mut guard = RUNTIME.outbound.lock().await;
	*guard = Some(mgr);
}

/// Register the event handler for an action defined in the plugin manifest
pub async fn register_action<A: Action>(action: A) {
	let uuid = A::UUID;
	let mut actions = RUNTIME.actions.lock().await;
	if actions.contains_key(uuid) {
		log::warn!("Action '{}' already registered", uuid);
		return;
	}
	actions.insert(uuid, Arc::new(ActionWrapper(action)));
	RUNTIME.visible.insert(uuid, DashSet::new());
}

async fn get_action(uuid: &str) -> Option<Arc<dyn ErasedAction>> {
	let actions = RUNTIME.actions.lock().await;
	actions.get(uuid).cloned()
}

async fn resolve(action_uuid: &str, instance_id: &str) -> Result<Option<(Arc<dyn ErasedAction>, Arc<Instance>)>> {
	let Some(action) = get_action(action_uuid).await else {
		return Ok(None);
	};
	let Some(instance) = RUNTIME.instances.get(instance_id) else {
		return Ok(None);
	};
	Ok(Some((action, instance.clone())))
}

pub(crate) async fn handle_will_appear(event: AppearEvent) -> Result<()> {
	let Some(action) = get_action(&event.action).await else {
		log::warn!("Unknown action '{}' (willAppear)", event.action);
		return Ok(());
	};

	let instance_id = event.context.clone();
	let existing = RUNTIME.instances.get(&instance_id).map(|a| a.clone());

	let instance = if let Some(ins) = existing {
		ins
	} else {
		let ins = Arc::new(Instance {
			action_uuid: event.action.clone(),
			instance_id: instance_id.clone(),
			device_id: event.device.clone(),
			controller: event.payload.controller.clone(),
			coordinates: event.payload.coordinates,
			is_in_multi_action: event.payload.is_in_multi_action,
			current_state_index: std::sync::atomic::AtomicU16::new(event.payload.state),
			settings_json: RwLock::new(event.payload.settings.clone()),
		});
		RUNTIME.instances.insert(instance_id.clone(), ins.clone());
		ins
	};

	RUNTIME.visible.entry(action.uuid()).or_default().insert(instance_id);

	action.call_will_appear(&instance, event.payload).await
}

pub(crate) async fn handle_will_disappear(event: AppearEvent) -> Result<()> {
	let instance = if let Some(entry) = RUNTIME.instances.remove(&event.context) {
		entry.1
	} else {
		return Ok(());
	};
	if let Some(action) = get_action(&event.action).await {
		action.call_will_disappear(&instance, event.payload).await?;
		if let Some(set) = RUNTIME.visible.get(&action.uuid()) {
			set.remove(&event.context);
		}
	}
	Ok(())
}

/// List all instances of an action currently visible to the user
pub async fn visible_instances(action_uuid: ActionUuid) -> Vec<Arc<Instance>> {
	RUNTIME
		.visible
		.get(&action_uuid)
		.map(|set| {
			set.iter()
				.filter_map(|id| RUNTIME.instances.get(&*id).as_deref().cloned())
				.collect()
		})
		.unwrap_or_default()
}

/// Get an instance of an action by its ID
pub async fn get_instance(instance_id: InstanceId) -> Option<Arc<Instance>> {
	RUNTIME.instances.get(&instance_id).as_deref().cloned()
}

pub(crate) static CONNECTED_DEVICES: LazyLock<DashMap<String, crate::inbound::DeviceInfo>> =
	LazyLock::new(DashMap::new);

/// List all connected devices
pub async fn get_connected_devices() -> HashMap<String, crate::inbound::DeviceInfo> {
	CONNECTED_DEVICES
		.iter()
		.map(|entry| (entry.key().clone(), entry.value().clone()))
		.collect()
}
