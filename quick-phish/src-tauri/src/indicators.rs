use serde::Serialize;
use serde_json::json;
use std::collections::HashSet;

#[derive(Serialize)]
pub struct Indicators {
    pub urls: HashSet<String>,
    pub emails: HashSet<String>,
    pub ips: HashSet<String>,
    pub domains: HashSet<String>,
}

impl Indicators {
    pub fn new() -> Self {
        Self {
            urls: HashSet::new(),
            emails: HashSet::new(),
            ips: HashSet::new(),
            domains: HashSet::new(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({ "urls": self.urls, "emails": self.emails, "ips": self.ips, "domains": self.domains })
    }
}
