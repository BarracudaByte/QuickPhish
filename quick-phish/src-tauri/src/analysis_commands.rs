use eml_parser::eml::{EmailAddress, HeaderFieldValue};
use eml_parser::parser::EmlParser;
use linkify::{LinkFinder, LinkKind};
use minijinja::{Environment, context};
use serde_json::json;
use tauri::AppHandle;

use crate::indicators::Indicators;
use crate::parsed_eml::ParsedEml;
use crate::template_commands::{get_template, SUMARY_TEMPLATE};


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

fn find_iocs(text: &str, with_scheme: bool) -> Indicators {
    let mut indicators = Indicators::new();
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(with_scheme);
    finder.links(text).for_each(|link| {
        if *link.kind() == LinkKind::Url {
            indicators.urls.push(link.as_str().to_string())
        } else if *link.kind() == LinkKind::Email {
            indicators.emails.push(link.as_str().to_string())
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

#[tauri::command]
pub fn load_eml(uri: &str, app_handle: AppHandle) -> serde_json::Value { // HashMap<String, String>
    //let mut data: HashMap<String, String> = HashMap::new();
    //let data = std::fs::read(&uri).unwrap();
    // tauri::ipc::Response::new(eml) -> use with return Response
    let eml = EmlParser::from_file(&uri).unwrap().parse().unwrap(); // .ignore_body()
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

    // Template Logic
    let summary: String = render_summary(&parsed_eml, &app_handle);
    let parsed_eml_json = parsed_eml.to_json_with(summary);

    return parsed_eml_json;
}
