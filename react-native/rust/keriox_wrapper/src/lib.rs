mod export;
use jolocom_native_utils::{get_icp_str, validate_events_str};

export! {
    @Java_io_jolocom_nativeUtils_NativeUtilsModule_getIcp
    // returns a signed versioned key event with the keys
    fn get_icp() -> String {
        get_icp_str()
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_validateEvents
    fn validate_events(kel_string: String) -> String {
        validate_events_str(kel_string)
    }
}
