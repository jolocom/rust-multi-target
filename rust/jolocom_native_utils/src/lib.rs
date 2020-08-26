pub mod did_document;
pub mod wallet;
use did_document::{state_to_did_document, DIDDocument};
use keri::{
    error::Error,
    event_message::{parse_signed_message_json, validate_events, EventMessage},
    prefix::Prefix,
};

pub fn validate_events_str(kel_str: &str, method_name: &str) -> Result<String, String> {
    let str_events: Vec<String> = match serde_json::from_str(&kel_str) {
        Ok(k) => k,
        Err(e) => return Err(e.to_string()),
    };
    let kel: Vec<EventMessage> = match str_events
        .iter()
        .map(|e| parse_signed_message_json(e))
        .collect::<Result<Vec<EventMessage>, Error>>()
    {
        Ok(k) => k,
        Err(e) => return Err(e.to_string()),
    };

    let ddo: DIDDocument = match validate_events(&kel) {
        Ok(s) => state_to_did_document(s, method_name),
        Err(e) => return Err(e.to_string()),
    };

    match serde_json::to_string(&ddo) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_id_from_event_str(event: &str) -> Result<String, String> {
    match parse_signed_message_json(event) {
        Ok(ev) => Ok(ev.event.prefix.to_str()),
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn test() {}
