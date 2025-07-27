use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

use crate::indicators::Indicators;

 
#[derive(Serialize)]
pub struct ParsedEml {
    pub headers: HashMap<String, String>,
    pub body: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub iocs: Indicators
}

impl ParsedEml {
    pub fn new(body: String, from: String, to: String, subject: String, indicators: Indicators) -> Self {
        Self {
            headers: HashMap::new(),
            body: body,
            from: from,
            to: to,
            subject: subject,
            iocs: indicators,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({ "headers": self.headers, "body": self.body, "from": self.from, "to": self.to, "subject": self.subject, "indicators": self.iocs.to_json() })
    }

    pub fn to_json_with(&self, summary: String) -> serde_json::Value {
        json!({ "headers": self.headers, "body": self.body, "from": self.from, "to": self.to, "subject": self.subject, "indicators": self.iocs.to_json(), "summary": summary })
    }
}