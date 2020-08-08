pub mod did_document;
pub mod wallet;
use did_document::DIDDocument;
use keriox::{
    error::Error,
    event_message::{parse_signed_message, validate_events, VersionedEventMessage},
};

pub fn validate_events_str(kel_str: &str) -> String {
    let str_events: Vec<String> = match serde_json::from_str(&kel_str) {
        Ok(k) => k,
        Err(e) => return e.to_string(),
    };
    let kel: Vec<VersionedEventMessage> = match str_events
        .iter()
        .map(|e| parse_signed_message(e))
        .collect::<Result<Vec<VersionedEventMessage>, Error>>(
    ) {
        Ok(k) => k,
        Err(e) => return e.to_string(),
    };

    let ddo: DIDDocument = match validate_events(&kel) {
        Ok(s) => s.into(),
        Err(e) => return e.to_string(),
    };

    match serde_json::to_string(&ddo) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}

pub fn get_id_from_event_str(event: &str) -> String {
    match parse_signed_message(event) {
        Ok(vem) => match vem {
            VersionedEventMessage::V0_0(ev) => ev.event.prefix.to_str(),
        },
        Err(e) => e.to_string(),
    }
}

#[test]
fn test() {}
