use base64;
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json;
use wallet_rs::{get_random, prelude::*};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddKeyResultRep {
    pub new_encrypted_state: String,
    pub new_key: ContentEntity,
}

pub fn get_random_b64(len: usize) -> String {
    match get_random(len) {
        Ok(rb) => base64::encode_config(rb, base64::URL_SAFE),
        Err(e) => e,
    }
}

pub fn wallet_from(encrypted_wallet: &str, id: &str, pass: &str) -> Result<UnlockedWallet, String> {
    let ew = match base64::decode_config(encrypted_wallet, base64::URL_SAFE) {
        Ok(w) => w,
        Err(e) => return Err(e.to_string()),
    };
    let lw = LockedWallet::new(id, ew);

    lw.unlock(pass.as_bytes())
}

pub fn new_wallet(id: &str, pass: &str) -> String {
    let uw = UnlockedWallet::new(&id);
    export_wallet(uw, &pass)
}

pub fn export_wallet(uw: UnlockedWallet, pass: &str) -> String {
    match uw.lock(pass.as_bytes()) {
        Ok(lw) => base64::encode_config(&lw.ciphertext, base64::URL_SAFE),
        Err(e) => e,
    }
}

pub fn change_pass(encrypted_wallet: &str, id: &str, old_pass: &str, new_pass: &str) -> String {
    let uw = match wallet_from(encrypted_wallet, id, &old_pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };
    export_wallet(uw, new_pass)
}

pub fn new_key(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_type: &str,
    controller: Option<Vec<String>>,
) -> String {
    let mut uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let nkt = match KeyType::from_str(key_type) {
        Ok(kt) => kt,
        Err(e) => return e,
    };

    let key = match uw.new_key(nkt, controller) {
        Ok(r) => r,
        Err(e) => return e,
    };

    match serde_json::to_string(&AddKeyResultRep {
        new_encrypted_state: export_wallet(uw, pass),
        new_key: key,
    }) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}

pub fn sign(encrypted_wallet: &str, id: &str, pass: &str, key_ref: &str, data: &str) -> String {
    let uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let data_bytes = match base64::decode_config(data, base64::URL_SAFE) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    let sig = match uw.sign_raw(key_ref, &data_bytes) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    base64::encode_config(sig, base64::URL_SAFE)
}

pub fn verify(pk_info_str: &str, data: &str, sig: &str) -> bool {
    let pk_info: ContentEntity = match serde_json::from_str(pk_info_str) {
        Ok(k) => k,
        Err(_) => return false,
    };

    let data_bytes = match base64::decode_config(&data, base64::URL_SAFE) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let sig_bytes = match base64::decode_config(sig, base64::URL_SAFE) {
        Ok(s) => s,
        Err(_) => return false,
    };

    match pk_info.content {
        Content::PublicKey(pk) => match pk.verify(&data_bytes, &sig_bytes) {
            Ok(v) => v,
            Err(_) => false,
        },
        _ => false,
    }
}

pub fn decrypt(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    data: &str,
    aad: &str,
) -> String {
    let uw = match wallet_from(encrypted_wallet, id, &pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let data_bytes = match base64::decode_config(data, base64::URL_SAFE) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    let aad_bytes = match base64::decode_config(aad, base64::URL_SAFE) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    let decrypted = match uw.decrypt(key_ref, &data_bytes, &aad_bytes) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    base64::encode_config(decrypted, base64::URL_SAFE)
}

pub fn encrypt(pk_info_str: &str, data: &str, aad: &str) -> String {
    let pk: PublicKeyInfo = match serde_json::from_str::<ContentEntity>(pk_info_str) {
        Ok(k) => match k.content {
            Content::PublicKey(cpk) => cpk,
            _ => return "Wrong Key Type".to_string(),
        },
        Err(e) => return e.to_string(),
    };

    let data_bytes = match base64::decode_config(data, base64::URL_SAFE) {
        Ok(b) => b,
        Err(e) => return e.to_string(),
    };

    let aad_bytes = match base64::decode_config(aad, base64::URL_SAFE) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    match pk.encrypt(&data_bytes, &aad_bytes) {
        Ok(v) => base64::encode_config(v, base64::URL_SAFE),
        Err(e) => e.to_string(),
    }
}

pub fn get_keys(encrypted_wallet: &str, id: &str, pass: &str) -> String {
    let uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    match serde_json::to_string(&uw.get_keys()) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}

pub fn get_key(encrypted_wallet: &str, id: &str, pass: &str, key_ref: &str) -> String {
    let uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    match serde_json::to_string(&uw.get_key(&key_ref)) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}

#[test]
fn test() -> Result<(), String> {
    let id = "my_did".to_string();
    let p = "my_password".to_string();

    let ew = new_wallet(&id, &p);

    let ew_k1 = new_key(&ew, &id, &p, "EcdsaSecp256k1VerificationKey2019", None);

    let keys = get_keys(&ew_k1, &id, &p);

    Ok(())
}
