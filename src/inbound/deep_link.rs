use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DidReceiveDeepLinkPayload {
	pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DidReceiveDeepLinkEvent {
	pub payload: DidReceiveDeepLinkPayload,
}
