mod export;

export! {
    @Java_io_parity_substrateSign_SubstrateSignModule_getIcp
    // returns a signed versioned key event with the keys
    fn get_icp() -> String {
        String::from("get_icp")
    }

    @Java_io_parity_substrateSign_SubstrateSignModule_validateEvents
    fn validate_events(kel_string: String) -> String {
        String::from("validate_events")
    }
}
