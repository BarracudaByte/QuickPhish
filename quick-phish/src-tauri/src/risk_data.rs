use serde::Serialize;
use serde_json::json;


#[derive(Serialize)]
pub struct RiskScore {
    pub score: u8,
    pub reasons: Vec<String>
}

impl RiskScore {
    pub fn new() -> Self {
        Self {
            score: 0,
            reasons: Vec::new()
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({ "score": self.score, "reasons": self.reasons })
    }
}