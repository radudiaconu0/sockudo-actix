use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::collections::HashMap;

fn true_() -> bool {
    true
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub id: String, //These can't be null
    pub key: String,
    pub secret: Option<String>,
    pub max_connections: Option<u64>,
    #[serde(default = "true_")]
    pub enable_client_messages: bool,
    #[serde(default = "true_")]
    pub enabled: bool,
    pub max_backend_events_per_second: Option<u64>,
    pub max_client_events_per_second: Option<u64>,
    pub max_read_requests_per_minute: Option<u64>,
    #[serde(default)]
    pub webhooks: Vec<Value>,
    pub max_presence_member_size_in_kb: Option<u64>,
    pub max_channel_name_length: Option<u64>,
    pub max_event_channel_at_once: Option<u64>,
    pub max_event_name_length: Option<u64>,
    pub max_event_payload_in_kb: Option<u64>,
    pub max_event_batch_size: Option<u64>,
    #[serde(default = "true_")]
    pub enable_user_authentication: bool,
    #[serde(default)]
    pub has_client_event_webhooks: bool,
    #[serde(default)]
    pub has_channel_occupied_webhooks: bool,
    #[serde(default)]
    pub has_channel_vacated_webhooks: bool,
    #[serde(default)]
    pub has_member_added_webhooks: bool,
    #[serde(default)]
    pub has_member_removed_webhooks: bool,
    #[serde(default)]
    pub has_cache_missed_webhooks: bool,
}