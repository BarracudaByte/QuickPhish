use serde::Serialize;
use serde_json::json;

 
#[derive(Serialize)]
pub struct Indicators {
    pub urls: Vec<String>,
    pub emails: Vec<String>
}

impl Indicators {
    pub fn new() -> Self {
        Self {
            urls: Vec::new(),
            emails: Vec::new(),
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({ "urls": self.urls, "emails": self.emails })
    }
}