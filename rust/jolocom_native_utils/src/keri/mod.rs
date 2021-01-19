use crate::error::Error;
use keri::{
    database::lmdb::LmdbEventDatabase,
    event_message::parse::{signed_event_stream, signed_message, Deserialized},
    prefix::{IdentifierPrefix, Prefix},
    processor::EventProcessor,
    state::IdentifierState,
};
use std::path::Path;

fn get_processor(path: &str) -> Result<EventProcessor<LmdbEventDatabase>, Error> {
    Ok(EventProcessor::new(
        LmdbEventDatabase::new(Path::new(path)).map_err(|e| Error::Generic(e.to_string()))?,
    ))
}

pub fn process_events(kel: &[u8], db_path: &str) -> Result<(), Error> {
    let events = signed_event_stream(kel).map_err(|e| Error::Generic(e.to_string()))?;
    let proc = get_processor(db_path)?;

    for event in events.1 {
        proc.process(event);
    }

    Ok(())
}

pub fn get_kel(id: &IdentifierPrefix, db_path: &str) -> Result<Option<Vec<u8>>, Error> {
    todo!()
}

pub fn get_state(id: &IdentifierPrefix, db_path: &str) -> Result<Option<IdentifierState>, Error> {
    get_processor(db_path)?
        .compute_state(id)
        .map_err(|e| Error::Generic(e.to_string()))
}
