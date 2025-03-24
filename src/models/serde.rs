use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct IPResponsePayload {
    pub(crate) ip: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) url: String,
    pub(crate) ip: String,
    pub(crate) api_key: String,
}

#[derive(Serialize, Clone, Default)]
pub(crate) struct UpdatePayload {
    pub(crate) content: String,
    pub(crate) name: String,
    pub(crate) proxied: Option<bool>,
    pub(crate) r#type: String,
    pub(crate) comment: Option<String>,
    pub(crate) tags: Option<Vec<String>>,
    pub(crate) ttl: Option<i16>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct UpdateResponseMessage {
    pub(crate) code: u64,
    pub(crate) message: String,
}
#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct UpdateResponseResultMeta {
    pub(crate) auto_added: Option<bool>,
    pub(crate) source: Option<String>,
}
#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct UpdateResponseResult {
    pub(crate) content: String,
    pub(crate) name: String,
    pub(crate) proxied: Option<bool>,
    pub(crate) r#type: String,
    pub(crate) comment: Option<String>,
    pub(crate) created_on: String,
    pub(crate) id: String,
    pub(crate) locked: Option<bool>,
    pub(crate) meta: Option<UpdateResponseResultMeta>,
    pub(crate) modified_on: String,
    pub(crate) proxiable: bool,
    pub(crate) tags: Option<Vec<String>>,
    pub(crate) ttl: Option<u64>,
    pub(crate) zone_id: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub(crate) struct UpdateResponse {
    pub(crate) errors: Vec<UpdateResponseMessage>,
    pub(crate) messages: Option<Vec<UpdateResponseMessage>>,
    pub(crate) success: bool,
    pub(crate) result: Option<UpdateResponseResult>,
}
