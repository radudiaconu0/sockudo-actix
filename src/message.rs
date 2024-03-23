use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// For fields that can be of any type, you might need a custom enum or use serde_json::Value
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AnyValue {
    String(String),
    Map(HashMap<String, serde_json::Value>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageData {
    pub channel_data: Option<String>,
    pub channel: Option<String>,
    pub user_data: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>, // For additional dynamic fields
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PusherMessage {
    pub channel: Option<String>,
    pub name: Option<String>,
    pub event: String,
    pub data: Option<MessageData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PusherApiMessage {
    pub name: Option<String>,
    pub data: Option<String>,
    pub channel: Option<String>,
    pub channels: Option<Vec<String>>,
    pub socket_id: Option<String>,
    pub info: Option<PusherApiMessageInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PusherApiMessageInfo {
    pub user_count: Option<u64>,
    pub subscription_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentPusherMessage {
    pub channel: Option<String>,
    pub event: Option<String>,
    pub data: Option<AnyValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebSocketMessage {
    ArrayBuffer(Vec<u8>),
    PusherMessage(PusherMessage),
}