use serde_json::Value;

pub type Session<'a> = rocket_session::Session<'a, serde_json::Map<String, Value>>;