use base64;
use wallet_rs::{get_random, prelude::*};

pub fn get_random_b64(len: usize) -> String {
    match get_random(len) {
        Ok(rb) => base64::encode_config(rb, base64::URL_SAFE),
        Err(e) => e,
    }
}

pub fn wallet_from(
    encrypted_wallet: String,
    id: String,
    pass: &str,
) -> Result<UnlockedWallet, String> {
    let ew = match base64::decode_config(&encrypted_wallet, base64::URL_SAFE) {
        Ok(w) => w,
        Err(e) => return Err(e.to_string()),
    };
    let lw = LockedWallet::new(id, ew);

    lw.unlock(pass.as_bytes())
}

pub fn new_wallet(id: String, pass: String) -> String {
    let uw = UnlockedWallet::new(&id);
    export_wallet(uw, &pass)
}

pub fn export_wallet(uw: UnlockedWallet, pass: &str) -> String {
    match uw.lock(pass.as_bytes()) {
        Ok(lw) => base64::encode_config(&lw.ciphertext, base64::URL_SAFE),
        Err(e) => e,
    }
}

pub fn change_pass(
    encrypted_wallet: String,
    id: String,
    old_pass: String,
    new_pass: String,
) -> String {
    let uw = match wallet_from(encrypted_wallet, id, &old_pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };
    export_wallet(uw, &new_pass)
}

pub fn new_key(
    encrypted_wallet: String,
    id: String,
    pass: String,
    key_type: String,
    controller: Option<Vec<String>>,
) -> String {
    let mut uw = match wallet_from(encrypted_wallet, id, &pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let nkt = match serde_json::from_str::<KeyType>(&key_type) {
        Ok(kt) => kt,
        Err(e) => return e.to_string(),
    };

    let _ref = match uw.new_key(nkt, controller) {
        Ok(r) => r,
        Err(e) => return e,
    };

    export_wallet(uw, &pass)
}

pub fn sign(
    encrypted_wallet: String,
    id: String,
    pass: String,
    data: String,
    key_ref: String,
) -> String {
    let uw = match wallet_from(encrypted_wallet, id, &pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let sig = match uw.sign_raw(data.as_bytes(), &key_ref) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    base64::encode_config(sig, base64::URL_SAFE)
}

pub fn verify(
    encrypted_wallet: String,
    id: String,
    pass: String,
    data: String,
    key_ref: String,
    sig: String,
) -> bool {
    let uw = match wallet_from(encrypted_wallet, id, &pass) {
        Ok(w) => w,
        Err(_) => return false,
    };

    let sig_bytes = match base64::decode_config(&sig, base64::URL_SAFE) {
        Ok(s) => s,
        Err(_) => return false,
    };

    match uw.verify_raw(data.as_bytes(), &key_ref, &sig_bytes) {
        Ok(v) => v,
        Err(_) => false,
    }
}

pub fn get_keys(encrypted_wallet: String, id: String, pass: String) -> String {
    let uw = match wallet_from(encrypted_wallet, id, &pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    match serde_json::to_string(&uw.get_keys()) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}
