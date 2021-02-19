use crate::error::Error;
use keri::{
    database::lmdb::LmdbEventDatabase,
    event_message::parse::{signed_event_stream, signed_message, Deserialized},
    prefix::{IdentifierPrefix, Prefix},
    processor::EventProcessor,
    state::IdentifierState,
};
use std::fs::create_dir_all;
use std::path::Path;

fn get_processor(path_str: &str) -> Result<EventProcessor<LmdbEventDatabase>, Error> {
    let path = Path::new(path_str);
    if !path.exists() {
        create_dir_all(path).map_err(|e| Error::Generic(e.to_string()))?;
    }
    Ok(EventProcessor::new(
        LmdbEventDatabase::new(path).map_err(|e| Error::Generic(e.to_string()))?,
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

pub fn get_kerl(id: &IdentifierPrefix, db_path: &str) -> Result<Option<Vec<u8>>, Error> {
    let proc = get_processor(db_path)?;
    proc.get_kerl(id).map_err(|e| Error::Generic(e.to_string()))
}

pub fn get_state(id: &IdentifierPrefix, db_path: &str) -> Result<Option<IdentifierState>, Error> {
    get_processor(db_path)?
        .compute_state(id)
        .map_err(|e| Error::Generic(e.to_string()))
}

#[test]
fn basic_test() {
    use std::fs;
    use tempfile::Builder;

    let root = Builder::new().prefix("test-db").tempdir().unwrap();
    fs::create_dir_all(root.path()).unwrap();

    let event = br#"{"v":"KERI10JSON000144_","i":"EJPRBUSEdUuZnh9kRGg8y7uBJDxTGZdp4YeUSqBv5sEk","s":"0","t":"icp","kt":"2","k":["DSuhyBcPZEZLK-fcw5tzHn2N46wRCG_ZOoeKtWTOunRA","DVcuJOOJF1IE8svqEtrSuyQjGTd2HhfAkt9y2QkUtFJI","DT1iAhBWCkvChxNWsby2J0pJyxBIxbAtbLA0Ljx-Grh8"],"n":"E9izzBkXX76sqt0N-tfLzJeRqj0W56p4pDQ_ZqNCDpyw","wt":"0","w":[],"c":[]}-AADAA74a3kHBjpaY2h3AzX8UursaGoW8kKU1rRLlMTYffMvKSTbhHHy96brGN2P6ehcmEW2nlUNZVuMf8zo6Qd8PkCgABIJfoSJejaDh1g-UZKkldxtTCwic7kB3s15EsDPKpm_6EhGcxVTt0AFXQUQMroKgKrGnxL0GP6gwEdmdu9dVRAgACtJFQBQiRX5iqWpJQntfAZTx6VIv_Ghydg1oB0QCq7s8D8LuKH5n1S5t8AbbQPXv6Paf7AVJRFv8lhCT5cdx3Bg"#;
    let id: IdentifierPrefix = "EJPRBUSEdUuZnh9kRGg8y7uBJDxTGZdp4YeUSqBv5sEk"
        .parse()
        .unwrap();
    let path = root.path().to_str().unwrap();

    assert_eq!(process_events(event, path).unwrap(), ());

    assert_eq!(get_state(&id, path).unwrap().unwrap().prefix, id);
}
