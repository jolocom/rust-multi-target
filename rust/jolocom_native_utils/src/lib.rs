pub mod did_document;
pub mod wallet;
pub mod error;
pub use error::Error;
use did_document::{state_to_did_document, DIDDocument};
use keri::{
    event_message::parse::{signed_event_stream_validate, signed_message, Deserialized},
    prefix::Prefix,
};
use universal_wallet::Error as UwError;

pub fn validate_events_str<'a>(kel_str: &[u8], method_name: &str) -> Result<String, Error> {
    let state = signed_event_stream_validate(kel_str)
        .map_err(|e| Error::Generic(e.to_string()))?;
    let ddo: DIDDocument = state_to_did_document(state.1, method_name);

    Ok(serde_json::to_string(&ddo).map_err(|e| UwError::Serde(e))?)
}

pub fn get_id_from_event_str(event: &[u8]) -> Result<String, Error> {
    match signed_message(event) {
        Ok((_, ev)) => match ev {
            Deserialized::Event(e) => Ok(e.event.event.event.prefix.to_str()),
            Deserialized::Rct(r) => Ok(r.body.event.prefix.to_str()),
            Deserialized::Vrc(r) => Ok(r.event_message.event.prefix.to_str())
        }
        Err(e) => Err(Error::Generic(e.to_string())),
    }
}

#[test]
fn test() {}
