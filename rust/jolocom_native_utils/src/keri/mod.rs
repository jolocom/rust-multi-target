use crate::error::Error;
use keri::{
    database::lmdb::LmdbEventDatabase,
    event_message::parse::{signed_event_stream, signed_message, Deserialized},
    prefix::{IdentifierPrefix, Prefix},
    processor::EventProcessor,
    state::IdentifierState,
};
use std::path::Path;

fn get_processor() -> Result<EventProcessor<LmdbEventDatabase>, Error> {
    let path_str: &'static str = env!("EVENT_DB_PATH", "No Event Database Path provided");
    Ok(EventProcessor::new(
        LmdbEventDatabase::new(Path::new(path_str)).map_err(|e| Error::Generic(e.to_string()))?,
    ))
}

pub fn process_events(kel: &[u8]) -> Result<(), Error> {
    let events = signed_event_stream(kel).map_err(|e| Error::Generic(e.to_string()))?;
    let proc = get_processor()?;

    for event in events.1 {
        proc.process(event);
    }

    Ok(())
}

pub fn get_kel(id: &IdentifierPrefix) -> Result<Option<Vec<u8>>, Error> {
    todo!()
}

pub fn get_state(id: &IdentifierPrefix) -> Result<Option<IdentifierState>, Error> {
    get_processor()?
        .compute_state(id)
        .map_err(|e| Error::Generic(e.to_string()))
}
