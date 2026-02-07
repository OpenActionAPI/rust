use super::DeviceInfo;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Info {
	pub devices: Vec<DeviceInfo>,
	// Other unused fields...
}
