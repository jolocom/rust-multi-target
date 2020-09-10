pub mod did_document;
pub mod wallet;
use did_document::{state_to_did_document, DIDDocument};
use keri::{
    event_message::parse::{signed_event_stream_validate, signed_message},
    prefix::Prefix,
};

pub fn validate_events_str(kel_str: &str, method_name: &str) -> Result<String, String> {
    let ddo: DIDDocument = match signed_event_stream_validate(&kel_str) {
        Ok((_, s)) => state_to_did_document(s, method_name),
        Err(e) => return Err(e.to_string()),
    };

    match serde_json::to_string(&ddo) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_id_from_event_str(event: &str) -> Result<String, String> {
    match signed_message(event) {
        Ok((_, ev)) => Ok(ev.event_message.event.prefix.to_str()),
        Err(e) => Err(e.to_string()),
    }
}

#[test]
fn test() {}
