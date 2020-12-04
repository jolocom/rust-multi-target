pub mod did_document;
pub mod wallet;
pub mod error;
pub use error::Error;
use did_document::{state_to_did_document, DIDDocument};
use keri::{
    event_message::parse::{signed_event_stream_validate, signed_message},
    prefix::Prefix,
    error::Error as KError,
};
use universal_wallet::Error as UwError;

pub fn validate_events_str(kel_str: &'static str, method_name: &str) -> Result<String, Error> {
    let state = signed_event_stream_validate(kel_str.as_bytes())
        .map_err(|e| KError::NomIResult(e))?;
    let ddo: DIDDocument = state_to_did_document(state.1, method_name);

    Ok(serde_json::to_string(&ddo).map_err(|e| UwError::Serde(e))?)
}

// TODO: Makes no sens? =(
pub fn get_id_from_event_str(event: &str) -> Result<String, Error> {
    match signed_message(event) {
        Ok((_, ev)) => Ok(ev.event_message.event.prefix.to_str()),
        Err(e) => Err(Error::Generic(e.to_string())),
    }
}

#[test]
fn test() {}
