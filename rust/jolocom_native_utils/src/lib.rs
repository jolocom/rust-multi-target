use keriox::{
    error::Error,
    event_message::{get_icp, parse_signed_message, validate_events, VersionedEventMessage},
};

pub fn get_icp_str() -> String {
    let icp_with_keys = match get_icp() {
        Ok(icp) => icp,
        Err(e) => return e.to_string(),
    };
    match serde_json::to_string(&icp_with_keys) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}

pub fn validate_events_str(kel: String) -> String {
    let str_events: Vec<String> = match serde_json::from_str(&kel) {
        Ok(k) => k,
        Err(e) => return e.to_string(),
    };
    let kel: Vec<VersionedEventMessage> = match str_events
        .iter()
        .map(|e| parse_signed_message(e.to_string()))
        .collect::<Result<Vec<VersionedEventMessage>, Error>>()
    {
        Ok(k) => k,
        Err(e) => return e.to_string(),
    };

    validate_events(&kel)
}

#[test]
fn test() {
    let tings = get_icp().unwrap();
    print!("\n--\n{}\n--\n", tings.icp);
    let nt = serde_json::to_string(&vec![tings.icp]).unwrap();
    print!("\n--\n{}\n--\n", nt);

    let ddo = validate_events_str(nt);
    print!("{}", serde_json::to_string(&ddo).unwrap());
}
