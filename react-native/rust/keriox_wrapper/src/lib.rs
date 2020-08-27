mod export;
use jolocom_native_utils::{get_id_from_event_str, validate_events_str, wallet};
use std::convert::TryInto;

export! {
    @Java_io_jolocom_jolocomCore_JolocomCoreModule_validateEventsStr
    fn validate_events(kel_string: String) -> Result<String, String> {
        validate_events_str(&kel_string, "jun")
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getIdFromEvent
    fn get_id_from_event(event_string: String) -> Result<String, String> {
        get_id_from_event_str(&event_string)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_newWalletStr
    fn new_wallet(id: String, pass: String) -> Result<String, String> {
        wallet::new_wallet(&id, &pass)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_keriInceptWalletStr
    fn keri_incept_wallet(ew: String, id: String, pass: String) -> Result<String, String> {
        wallet::incept_wallet(&ew, &id, &pass)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_keriInceptWalletFromKeysStr
    fn keri_incept_wallet_from_keys(live_keys: String, pre_rotated_keys: String, pass: String) -> Result<String, String> {
        wallet::incept_populated_wallet(&live_keys, &pre_rotated_keys, &pass)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_changePassStr
    fn change_pass(ew: String, id: String, old_pass: String, new_pass: String) -> Result<String, String> {
        wallet::change_pass(&ew, &id, &old_pass, &new_pass)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_changeIdStr
    fn change_id(ew: String, id: String, new_id: String, pass: String) -> Result<String, String> {
        wallet::change_id(&ew, &id, &new_id, &pass)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_newKeyStr
    fn new_key(ew: String, id: String, pass: String, key_type: String, controller: String) -> Result<String, String> {
        wallet::new_key(&ew, &id, &pass, &key_type, if controller.len() > 0 {Some(vec![controller])} else { None })
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_addContentStr
    fn add_content(ew: String, id: String, pass: String, content: String) -> Result<String, String> {
        wallet::add_content(&ew, &id, &pass, &content)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_setKeyControllerStr
    fn set_key_controller(ew: String, id: String, pass: String, key_ref: String, controller: String) -> Result<String, String> {
        wallet::set_key_controller(&ew, &id, &pass, &key_ref, &controller)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getKeyStr
    fn get_key(ew: String, id: String, pass: String, key_ref: String) -> Result<String, String> {
        wallet::get_key(&ew, &id, &pass, &key_ref)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getKeyByControllerStr
    fn get_key_by_controller(ew: String, id: String, pass: String, controller: String) -> Result<String, String> {
        wallet::get_key_by_controller(&ew, &id, &pass, &controller)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getKeysStr
    fn get_keys(ew: String, id: String, pass: String) -> Result<String, String> {
        wallet::get_keys(&ew, &id, &pass)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_signByControllerStr
    fn sign_by_controller(ew: String, id: String, pass: String, controller: String, data: String) -> Result<String, String> {
        wallet::sign_by_controller(&ew, &id, &pass, &controller, &data)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_verifyStr
    fn jc_verify(key: String, key_type: String, data: String, signature: String) -> Result<bool, String> {
        wallet::verify(&key, &key_type, &data, &signature)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_encryptStr
    fn jc_encrypt(key: String, key_type: String, data: String, aad: String) -> Result<String, String> {
        wallet::encrypt(&key, &key_type, &data, &aad)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_decryptStr
    fn jc_decrypt(ew: String, id: String, pass: String, key_ref: String, data: String, aad: String) -> Result<String, String> {
        wallet::decrypt_by_controller(&ew, &id, &pass, &key_ref, &data, &aad)
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getRandomStr
    fn get_random(len: u32) -> Result<String, String> {
        wallet::get_random_b64(len.try_into().unwrap())
    }
}

ffi_support::define_string_destructor!(jolo_destroy_string);
