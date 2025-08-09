use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct HeaderVerification {
    pub dkim: bool,
    pub spf: bool,
    pub arc: bool,
    pub dmarc: bool
}

impl HeaderVerification {
    pub fn new() -> Self {
        Self {
            dkim: false,
            spf: false,
            arc: false,
            dmarc: false
        }
    }
    pub fn to_json(&self) -> serde_json::Value {
        json!({ "dkim": self.dkim, "spf": self.spf, "arc": self.arc, "dmarc": self.dmarc })
    }
}