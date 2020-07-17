mod export;
use jolocom_native_utils::{
    get_icp_str,
    validate_events_str,
    get_id_from_event_str
};

export! {
    @Java_io_jolocom_nativeUtils_NativeUtilsModule_getIcpStr
    // returns a signed versioned key event with the keys
    fn get_icp() -> String {
        get_icp_str()
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_validateEventsStr
    fn validate_events(kel_string: String) -> String {
        validate_events_str(kel_string)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_extractIdFromEventStr
    fn get_id_from_event(event_string: String) -> String {
        get_id_from_event_str(event_string)
    }
}
