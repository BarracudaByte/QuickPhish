use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

use crate::header_verification::HeaderVerification;
use crate::indicators::Indicators;
use crate::risk_data::RiskScore;

#[derive(Serialize)]
pub struct ParsedEml {
    pub headers: HashMap<String, String>,
    pub body: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub indicators: Indicators,
}

impl ParsedEml {
    pub fn new(
        body: String,
        from: String,
        to: String,
        subject: String,
        indicators: Indicators,
    ) -> Self {
        Self {
            headers: HashMap::new(),
            body: body,
            from: from,
            to: to,
            subject: subject,
            indicators: indicators,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({ "headers": self.headers, "body": self.body, "from": self.from, "to": self.to, "subject": self.subject, "indicators": self.indicators.to_json() })
    }

    pub fn to_json_with(
        &self,
        summary: String,
        header_verification: HeaderVerification,
        score: RiskScore,
    ) -> serde_json::Value {
        json!({ "headers": self.headers, "body": self.body, "from": self.from, "to": self.to, "subject": self.subject, "indicators": self.indicators.to_json(), "summary": summary, "headerVerify": header_verification.to_json(), "riskScore": score.to_json() })
    }
}
