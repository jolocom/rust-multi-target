mod export;
use jolocom_native_utils::{get_id_from_event_str, validate_events_str, wallet};
use std::convert::TryInto;

export! {
    @Java_io_jolocom_nativeUtils_NativeUtilsModule_validateEventsStr
    fn validate_events(kel_string: String) -> String {
        validate_events_str(&kel_string)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_extractIdFromEventStr
    fn get_id_from_event(event_string: String) -> String {
        get_id_from_event_str(&event_string)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_newWalletStr
    fn new_wallet(id: String, pass: String) -> String {
        wallet::new_wallet(&id, &pass)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_keriInceptWalletStr
    fn keri_incept_wallet(ew: String, id: String, pass: String) -> String {
        wallet::incept_wallet(&ew, &id, &pass)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_changePassStr
    fn change_pass(ew: String, id: String, old_pass: String, new_pass: String) -> String {
        wallet::change_pass(&ew, &id, &old_pass, &new_pass)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_changeIdStr
    fn change_id(ew: String, id: String, new_id: String, pass: String) -> String {
        wallet::change_pass(&ew, &id, &new_id, &pass)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_newKeyStr
    fn new_key(ew: String, id: String, pass: String, key_type: String, controller: String) -> String {
        wallet::new_key(&ew, &id, &pass, &key_type, if controller.len() > 0 {Some(vec![controller])} else { None })
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_addContentStr
    fn add_content(ew: String, id: String, pass: String, content: String) -> String {
        wallet::add_content(&ew, &id, &pass, &content)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_setKeyControllerStr
    fn set_key_controller(ew: String, id: String, pass: String, key_ref: String, controller: String) -> String {
        wallet::set_key_controller(&ew, &id, &pass, &key_ref, &controller)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_getKeyStr
    fn get_key(ew: String, id: String, pass: String, key_ref: String) -> String {
        wallet::get_key(&ew, &id, &pass, &key_ref)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_getKeyByControllerStr
    fn get_key_by_controller(ew: String, id: String, pass: String, controller: String) -> String {
        wallet::get_key_by_controller(&ew, &id, &pass, &controller)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_getKeysStr
    fn get_keys(ew: String, id: String, pass: String) -> String {
        wallet::get_keys(&ew, &id, &pass)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_signStr
    fn sign(ew: String, id: String, pass: String, data: String, controller: String) -> String {
        wallet::sign(&ew, &id, &pass, &data, &controller)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_verifyStr
    fn verify(key: String, key_type: String, data: String, signature: String) -> bool {
        wallet::verify(&key, &key_type, &data, &signature)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_encryptStr
    fn encrypt(key: String, key_type: String, data: String, aad: String) -> String {
        wallet::encrypt(&key, &key_type, &data, &aad)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_decryptStr
    fn decrypt(ew: String, id: String, pass: String, key_ref: String, data: String, aad: String) -> String {
        wallet::decrypt(&ew, &id, &pass, &key_ref, &data, &aad)
    }

    @Java_io_jolocom_nativeUtils_NativeUtilsModule_getRandomStr
    fn get_random(len: u32) -> String {
        wallet::get_random_b64(len.try_into().unwrap())
    }

}
