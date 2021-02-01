mod export;
use jolocom_native_utils::{did_document, keri, wallet};
use serde_json::to_string;
use std::convert::TryInto;

export! {
    @Java_io_jolocom_jolocomCore_JolocomCoreModule_processEvents
    fn process_events(kel_string: String, db_path: String) -> Result<(), String> {
        keri::process_events(&kel_string.as_bytes(), &db_path).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_resolveId
    fn resolve_id(id: String, db_path: String, method_name: String) -> Result<String, String> {
        to_string(&did_document::state_to_did_document(
            keri::get_state(&id.parse().map_err(|_| "Invalid Identifier".to_string())?, &db_path)
                .map_err(|e| e.to_string())?
                .ok_or("Identifier Unregistered".to_string())?,
            &method_name
        ))
        .map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getKerl
    fn get_kerl(id: String, db_path: String) -> Result<String, String> {
        String::from_utf8(keri::get_kerl(&id.parse().map_err(|_| "Invalid Identifier".to_string())?, &db_path)
        .map_err(|e| e.to_string())?.ok_or("Identifier Unregistered")?).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_newWalletStr
    fn new_wallet(id: String, pass: String) -> Result<String, String> {
        wallet::new_wallet(&id, &pass).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_keriInceptWalletStr
    fn keri_incept_wallet(ew: String, id: String, pass: String) -> Result<String, String> {
        wallet::incept_wallet(&ew, &id, &pass).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_keriInceptWalletFromKeysStr
    fn keri_incept_wallet_from_keys(live_keys: String, pre_rotated_keys: String, pass: String) -> Result<String, String> {
        wallet::incept_populated_wallet(&live_keys, &pre_rotated_keys, &pass).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_changePassStr
    fn change_pass(ew: String, id: String, old_pass: String, new_pass: String) -> Result<String, String> {
        wallet::change_pass(&ew, &id, &old_pass, &new_pass).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_changeIdStr
    fn change_id(ew: String, id: String, new_id: String, pass: String) -> Result<String, String> {
        wallet::change_id(&ew, &id, &new_id, &pass).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_newKeyStr
    fn new_key(ew: String, id: String, pass: String, key_type: String, controller: String) -> Result<String, String> {
        wallet::new_key(&ew, &id, &pass, &key_type, if controller.len() > 0 {Some(vec![controller])} else { None }).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_addContentStr
    fn add_content(ew: String, id: String, pass: String, content: String) -> Result<String, String> {
        wallet::add_content(&ew, &id, &pass, &content).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_setKeyControllerStr
    fn set_key_controller(ew: String, id: String, pass: String, key_ref: String, controller: String) -> Result<String, String> {
        wallet::set_key_controller(&ew, &id, &pass, &key_ref, &controller).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getKeyStr
    fn get_key(ew: String, id: String, pass: String, key_ref: String) -> Result<String, String> {
        wallet::get_key(&ew, &id, &pass, &key_ref).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getKeyByControllerStr
    fn get_key_by_controller(ew: String, id: String, pass: String, controller: String) -> Result<String, String> {
        wallet::get_key_by_controller(&ew, &id, &pass, &controller).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getKeysStr
    fn get_keys(ew: String, id: String, pass: String) -> Result<String, String> {
        wallet::get_keys(&ew, &id, &pass).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_signByControllerStr
    fn sign_by_controller(ew: String, id: String, pass: String, controller: String, data: String) -> Result<String, String> {
        wallet::sign_by_controller(&ew, &id, &pass, &controller, &data).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_verifyStr
    fn jc_verify(key: String, key_type: String, data: String, signature: String) -> Result<bool, String> {
        wallet::verify(&key, &key_type, &data, &signature).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_encryptStr
    fn jc_encrypt(key: String, key_type: String, data: String, aad: String) -> Result<String, String> {
        wallet::encrypt(&key, &key_type, &data, &aad).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_decryptStr
    fn jc_decrypt(ew: String, id: String, pass: String, key_ref: String, data: String, aad: String) -> Result<String, String> {
        wallet::decrypt_by_controller(&ew, &id, &pass, &key_ref, &data, &aad).map_err(|e| e.to_string())
    }

    @Java_io_jolocom_jolocomCore_JolocomCoreModule_getRandomStr
    fn get_random(len: u32) -> Result<String, String> {
        wallet::get_random_b64(len.try_into().unwrap()).map_err(|e| e.to_string())
    }
}

ffi_support::define_string_destructor!(jolo_destroy_string);
