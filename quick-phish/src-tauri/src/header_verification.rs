use mail_auth::{DkimOutput, DkimResult, SpfResult};
use serde::Serialize;
use serde_json::json;
use std::collections::HashSet;

#[derive(Serialize, PartialEq, Eq, Hash, Debug)]
pub enum HeaderCheckResult {
    Pass,
    Fail(String),
    Neutral(String),
    Error(String),
    None
}

impl HeaderCheckResult  {
    pub fn from_vec(output: Vec<DkimOutput>) -> Vec<Self> {
        let mut result: HashSet<Self> = HashSet::new();
        for s in &output {
            result.insert(Self::from_result(s.result()));
        }
        if result.len() == 0 {
            return vec![HeaderCheckResult::None];
        }
        return result.into_iter().collect();
    }

    pub fn from_result(result: &DkimResult) -> Self {
        match result {
            DkimResult::Pass => {
                return HeaderCheckResult::Pass;
            },
            DkimResult::Neutral(e) => {
                return HeaderCheckResult::Neutral(e.to_string());
            },
            DkimResult::Fail(e) => {
                return HeaderCheckResult::Fail(e.to_string());
            },
            DkimResult::PermError(e) | DkimResult::TempError(e) => {
                return HeaderCheckResult::Error(e.to_string());
            },
            _ => {
                return HeaderCheckResult::None;
            }
        }
    } 

    pub fn from_spf_received(spf_received: String) -> Self {
        if spf_received.len() == 0 {
            return HeaderCheckResult::None;
        }
        println!("SPF-Received: {}", spf_received);
        let result = spf_received.split_whitespace().next().unwrap_or_default();
        
        // Extract the domain (e.g., "google.com")
        let domain_start = spf_received.find("domain of ");
        let mut domain: Option<&str> = None;
        if let Some(domain_start) = domain_start {
            let domain_end = spf_received[domain_start + 10..].find(' ');
            if let Some(domain_end) = domain_end {
                domain = Some(&spf_received[domain_start + 10..domain_start + 10 + domain_end]);
            }
        }

        // Extract Response
        let response_start = spf_received.find(" (");
        let mut response: &str = "";
        if let Some(response_start) = response_start {
            let response_end = spf_received[response_start + 2..].find(") ");
            if let Some(response_end) = response_end {
                response = &spf_received[response_start + 2..response_start + 2 + response_end];
            }
        }

        // Extract the client IP (e.g., "209.85.220.41")
        let client_ip_start = spf_received.find("client-ip=");
        let mut client_ip: Option<&str> = None;
        if let Some(client_ip_start) = client_ip_start {
            let client_ip_end = spf_received[client_ip_start + 10..].find(';');
            if let Some(client_ip_end) = client_ip_end {
                client_ip = Some(&spf_received[client_ip_start + 10..client_ip_start + 10 + client_ip_end]);
            }
        }
        if result.to_lowercase() == "pass" {
            return HeaderCheckResult::Pass;
        } else if result.to_lowercase().contains("fail") {
            match (client_ip, domain) {
                (Some(client_ip), Some(domain)) => {
                    return HeaderCheckResult::Fail(format!("{client_ip} is not a permitted sender of {domain}", ));
                },
                _ => {
                    return HeaderCheckResult::Fail(response.to_string());
                }
            }
            
        }
        /*match result {
            SpfResult::Pass => {
                return HeaderCheckResult::Pass;
            },
            SpfResult::Neutral => {
                return HeaderCheckResult::Neutral("".to_string());
            },
            SpfResult::Fail | SpfResult::SoftFail => {
                return HeaderCheckResult::Fail("".to_string());
            },
            SpfResult::PermError | SpfResult::TempError => {
                return HeaderCheckResult::Error("".to_string());
            },
            _ => {
                return HeaderCheckResult::None;
            }
        }*/
        return HeaderCheckResult::None
    }
}



#[derive(Serialize)]
pub struct HeaderVerification {
    pub dkim: Vec<HeaderCheckResult>,
    pub spf: HeaderCheckResult,
    pub arc: HeaderCheckResult,
    pub dmarc: bool,
}

impl HeaderVerification {
    pub fn new() -> Self {
        Self {
            dkim: vec!(),
            spf: HeaderCheckResult::None,
            arc: HeaderCheckResult::None,
            dmarc: false,
        }
    }
    pub fn to_json(&self) -> serde_json::Value {
        json!({ "dkim": self.dkim, "spf": self.spf, "arc": self.arc, "dmarc": self.dmarc })
    }
}
