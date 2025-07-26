use eml_parser::eml::{EmailAddress, HeaderFieldValue};
use eml_parser::parser::EmlParser;
use std::collections::HashMap;
use tauri::ipc::Response;
use tauri::{AppHandle, State};
use tauri_plugin_store::{StoreBuilder, Store};
 


fn parse_header_field_value(header_field: &HeaderFieldValue) -> Option<String> {
    match header_field {
        HeaderFieldValue::SingleEmailAddress(email) => match email {
            EmailAddress::AddressOnly { address } => Some(address.clone()),
            EmailAddress::NameAndEmailAddress { name, address } => Some(address.clone()),
        },
        HeaderFieldValue::MultipleEmailAddresses(emailAddresses) => None,
        HeaderFieldValue::Unstructured(value) => Some(value.clone()),
        HeaderFieldValue::Empty => None,
    }
}

#[tauri::command]
pub fn load_eml(uri: &str, app_handle: AppHandle) -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();
    //let data = std::fs::read(&uri).unwrap();

    // tauri::ipc::Response::new(eml) -> use with return Response
    let eml = EmlParser::from_file(&uri).unwrap().parse().unwrap(); // .ignore_body()
    eml.headers.iter().for_each(|header| {
        let value = parse_header_field_value(&header.value);
        data.insert(header.name.clone(), value.unwrap_or_default());
    });
    let from: HeaderFieldValue = eml.from.unwrap_or(HeaderFieldValue::Empty);
    let to: HeaderFieldValue = eml.to.unwrap_or(HeaderFieldValue::Empty);

    data.insert(
        "From".to_string(),
        parse_header_field_value(&from)
            .unwrap_or_default()
            .to_string(),
    );
    data.insert(
        "To".to_string(),
        parse_header_field_value(&to)
            .unwrap_or_default()
            .to_string(),
    );
    data.insert(
        "Subject".to_string(),
        eml.subject.unwrap_or_default().to_string(),
    );
    data.insert("Body".to_string(), eml.body.unwrap_or_default().to_string());
    return data;
}
