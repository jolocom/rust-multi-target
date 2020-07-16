use keriox::event_message::{get_icp, validate_events};

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
    validate_events(kel)
}
