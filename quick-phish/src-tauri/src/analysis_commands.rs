use eml_parser::eml::{EmailAddress, HeaderFieldValue, HeaderField};
use eml_parser::parser::EmlParser;
use linkify::{LinkFinder, LinkKind};
use mail_auth::{MessageAuthenticator, AuthenticatedMessage, DkimResult};
use mailparse::{MailHeaderMap, ParsedMail};
use minijinja::{Environment, context};
use serde_json::json;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::str;
use tauri::AppHandle;
use tauri::async_runtime::block_on;

use crate::indicators::Indicators;
use crate::risk_data::RiskScore;
use crate::header_verification::HeaderVerification;
use crate::parsed_eml::ParsedEml;
use crate::store_commands::{get_template, SUMARY_TEMPLATE};


fn parse_header_field_value(header_field: &HeaderFieldValue) -> Option<String> {
    match header_field {
        HeaderFieldValue::SingleEmailAddress(email) => match email {
            EmailAddress::AddressOnly { address } => Some(address.clone()),
            EmailAddress::NameAndEmailAddress { name: _, address } => Some(address.clone()),
        },
        HeaderFieldValue::MultipleEmailAddresses(_email_addresses) => None,    // TODO: Implement this!
        HeaderFieldValue::Unstructured(value) => Some(value.clone()),
        HeaderFieldValue::Empty => None,
    }
}

fn calculate_risk_score(iocs: &Indicators, from: Option<String>, headers: &Vec<HeaderField>, header_verification: &HeaderVerification) -> RiskScore {
    let mut risk_score = RiskScore::new();
    let mut score = 0;
    // if there are URLs in the email increase score
    let num_of_urls = iocs.urls.len();
    if num_of_urls > 0 {
        score += 25;
        risk_score.reasons.push(format!("Email has {num_of_urls} unique urls"))
    }
    // TODO: check how many unique URLs

    // if the From and Reply-To, Return-Path have different domains increase score
    let mut reply_to: Option<String> = None;
    let mut return_path: Option<String> = None;
    headers.iter().for_each(|header| {
        if header.name == "Reply-To" {
            reply_to = parse_header_field_value(&header.value);
        } else if header.name == "Return-Path" {
            return_path = parse_header_field_value(&header.value);
        }
    });

    if let Some(from) = from {
        let from_fqdn: &str = from.split("@").collect::<Vec<_>>()[1];

        // are the reply_to and the return_path domains the same as the from?
        if let Some(reply_to) = reply_to {
            if reply_to.ends_with(from_fqdn) {
                score -= 5;
                risk_score.reasons.push(format!("The reply to domain is the same as the sender (from) domain"));
            } else {
                score += 5;
                risk_score.reasons.push(format!("Different reply to domain than sender domain"));
            }
        }

        if let Some(return_path) = return_path {
            if return_path.ends_with(from_fqdn) {
                score -= 5;
                risk_score.reasons.push(format!("The return path domain is the same as the sender (from) domain"));
            } else {
                score += 5;
                risk_score.reasons.push(format!("Different return path domain than sender domain"));
            }
        }

        // if the sender domain is in the urls, reduce score
        if iocs.urls.contains(from_fqdn) {
            score -= 5;
            risk_score.reasons.push(format!("The sender (from) domain is also in the URLs"));
        }
    }

    // DKIM
    if !header_verification.dkim {
        let dkim_header = headers.iter().find(|&header| header.name == "DKIM-Signature");
        if dkim_header.is_some() {
            risk_score.reasons.push(format!("Invalid DKIM signature"));
        } else {
            risk_score.reasons.push(format!("No DKIM signature header"));
        }
        score += 5;
    }

    // ARC
    if !header_verification.arc {
        score += 5;
        risk_score.reasons.push(format!("Failed ARC chain verification"));
    }
    
    

    // updating the score
    risk_score.score = score;
    return risk_score;
}

fn find_iocs(text: &str, with_scheme: bool) -> Indicators {
    let mut indicators = Indicators::new();
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(with_scheme);
    finder.links(text).for_each(|link| {
        if *link.kind() == LinkKind::Url {
            indicators.urls.insert(link.as_str().to_string());
        } else if *link.kind() == LinkKind::Email {
            indicators.emails.insert(link.as_str().to_string());
        }
    });
    return indicators;
}

fn render_summary(eml: &ParsedEml, app_handle: &AppHandle) -> String {
    let default_template: &str = "This is the default tempalte";
    let binding = get_template(SUMARY_TEMPLATE, app_handle);
    let template: Option<&serde_json::Value> = binding.get("template");
    if template.is_some() {
        let template: &str = template.unwrap().as_str().unwrap_or(default_template);
        let mut env = Environment::new();
        env.add_template(SUMARY_TEMPLATE, template).unwrap();
        let template = env.get_template(SUMARY_TEMPLATE).unwrap();
        return template.render(context!(eml)).unwrap_or(default_template.to_string())
    }
    return default_template.to_string();
}

fn parse_body(eml: &ParsedMail, simple_text: bool) -> String {
    if eml.ctype.mimetype == "multipart/alternative" {
        if simple_text {
            let first_part = eml.subparts.first();
            if first_part.is_some() {
                return parse_body(first_part.unwrap(), simple_text);
            } else {
                return format!("No body I guess");
            }
        } else {
            let last_part = eml.subparts.last();
            if last_part.is_some() {
                return parse_body(last_part.unwrap(), simple_text);
            } else {
                return format!("No body I guess");
            }
        }
    } else {
        let body = match eml.get_body() {
            Ok(b) => b,
            Err(e) => format!("{:?}", e)
        };
        return body
    }
    
}

async fn verify_headers(msg: &Vec<u8>) -> HeaderVerification {
    let mut header_verification = HeaderVerification::new();
    let authenticator = MessageAuthenticator::new_cloudflare_tls().unwrap();
    let authenticated_message = AuthenticatedMessage::parse(msg).unwrap();

    // Validate DKIM Signature
    let result = authenticator.verify_dkim(&authenticated_message).await;
    let dkim_pass = result.iter().all(|s| s.result() == &DkimResult::Pass);
    header_verification.dkim = dkim_pass;

    // Validate ARC chain
    let result = authenticator.verify_arc(&authenticated_message).await;
    header_verification.arc = result.result() == &DkimResult::Pass;

    // TODO: add SPF & DMARC

    return header_verification;
}   


#[tauri::command]
pub fn load_eml(uri: &str, app_handle: AppHandle) -> serde_json::Value { 
    let file = File::open(uri);
    if file.is_err() {
        return json!({"error": "Coudln't open the file."});
    }
    let mut file = file.unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let read_result = file.read_to_end(&mut buffer);
    let parsed = mailparse::parse_mail(&buffer);
    let header_verification = block_on(verify_headers(&buffer));
    if parsed.is_err() {
        return json!({"error": "Coudln't parse the file."});
    }
    let parsed = parsed.unwrap(); 
    
    let contents = fs::read_to_string(&uri);
    if contents.is_err() {
        return json!({"error": "Coudln't open the file 2."});
    }
    let iocs: Indicators = find_iocs(&contents.unwrap(), true);



    /*parse_eml(&uri);

    let eml = EmlParser::from_file(&uri).ignore_body().unwrap().parse().unwrap(); // 
    let from: HeaderFieldValue = eml.from.unwrap_or(HeaderFieldValue::Empty);
    let to: HeaderFieldValue = eml.to.unwrap_or(HeaderFieldValue::Empty);
    let body = eml.body.unwrap_or_default();
    let subject = eml.subject.unwrap_or_default();
    let iocs: Indicators = find_iocs(&body, true);

    let from_parsed = parse_header_field_value(&from).unwrap_or_default();
    let to_parsed = parse_header_field_value(&to).unwrap_or_default();

    let mut parsed_eml = ParsedEml::new(body.to_string(), from_parsed.to_string(), to_parsed.to_string(), subject.to_string(), iocs);
    eml.headers.iter().for_each(|header| {
        // let value = parse_header_field_value(&header.value);
        // parsed_eml.headers.insert(header.name.clone(), value.unwrap_or_default());
        parsed_eml.headers.insert(header.name.clone(), header.value.to_string());
    });

    parsed_eml.headers.insert("From".to_string(), from.to_string());
    parsed_eml.headers.insert("To".to_string(), to.to_string());
    parsed_eml.headers.insert("Subject".to_string(), subject.to_string());
    */

    // TODO: better header parser: https://docs.rs/mailparse/latest/mailparse/fn.addrparse_header.html
    let subject = parsed.get_headers().get_first_value("Subject").unwrap_or_default();
    let from = parsed.get_headers().get_first_value("From").unwrap_or_default();
    let to = parsed.get_headers().get_first_value("To").unwrap_or_default();
    /*let body_maybe = String::from_utf8(parsed.get_body_raw().unwrap());
    let body = match body_maybe {
        Ok(b) => b.to_string(),
        Err(e) => format!("{:?}", e)
    };
    */
    let simple_text = false;
    let body = parse_body(&parsed, simple_text);

    // use different parser for easier headers for score
    let eml = EmlParser::from_file(&uri).unwrap().ignore_body().parse().unwrap(); 
    let from_parsed: Option<String> = parse_header_field_value(&eml.from.unwrap_or(HeaderFieldValue::Empty));
    let score = calculate_risk_score(&iocs, from_parsed, &eml.headers, &header_verification);
    //let body = format!("Subparts: {}. Content: {}", parsed.subparts.len(), parsed.ctype.mimetype);

    let mut parsed_eml = ParsedEml::new(body, from, to, subject, iocs);
    parsed.headers.iter().for_each(|header| {
        parsed_eml.headers.insert(header.get_key(), header.get_value());
    });

    // Template Logic
    let summary: String = render_summary(&parsed_eml, &app_handle);
    let parsed_eml_json = parsed_eml.to_json_with(summary, header_verification, score);
    
    return parsed_eml_json;
}
