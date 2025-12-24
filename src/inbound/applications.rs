use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationPayload {
	pub application: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationEvent {
	pub payload: ApplicationPayload,
}
