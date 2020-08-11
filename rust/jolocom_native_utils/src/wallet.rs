use base64;
use core::str::FromStr;
use keriox::{
    derivation::blake2b_256_digest,
    error::Error,
    event::{
        event_data::{inception::InceptionEvent, EventData},
        sections::{InceptionWitnessConfig, KeyConfig},
        Event,
    },
    event_message::{serialize_signed_message, EventMessage, VersionedEventMessage},
    prefix::Prefix,
    util::dfs_serializer,
};
use serde::{Deserialize, Serialize};
use serde_json;
use wallet_rs::{get_random, prelude::*};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddKeyResultRep {
    pub new_encrypted_state: String,
    pub new_key: ContentEntity,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletInceptionRep {
    pub id: String,
    pub encrypted_wallet: String,
    pub inception_event: String,
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

pub fn incept_wallet(encrypted_wallet: &str, id: &str, pass: &str) -> String {
    let mut uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e,
    };

    let nk0 = match uw.new_key(KeyType::Ed25519VerificationKey2018, None) {
        Ok(k) => k,
        Err(e) => return e,
    };

    let pref0 = match &nk0.content {
        Content::PublicKey(pk) => Prefix::PubKeyEd25519(pk.public_key.clone()),
        _ => return "Wrong Content Type".to_string(),
    };

    uw.id = ["did:un", &pref0.to_string()].join(":");
    uw.set_key_controller(
        &nk0.id,
        &[
            uw.id.clone(),
            base64::encode_config(pref0.derivative(), base64::URL_SAFE),
        ]
        .join("#"),
    );

    let nk1 = match uw.new_key(KeyType::Ed25519VerificationKey2018, None) {
        Ok(k) => k,
        Err(e) => return e,
    };

    let pref1 = match &nk1.content {
        Content::PublicKey(pk) => Prefix::Blake2B256(blake2b_256_digest(pk.public_key.as_ref())),
        _ => return "Wrong Content Type".to_string(),
    };

    let icp = VersionedEventMessage::V0_0(EventMessage {
        event: Event {
            prefix: pref0.clone(),
            sn: 0,
            event_data: EventData::Icp(InceptionEvent {
                key_config: KeyConfig {
                    threshold: 1,
                    public_keys: vec![pref0.clone()],
                    threshold_key_digest: pref1.clone(),
                },
                witness_config: InceptionWitnessConfig {
                    tally: 0,
                    initial_witnesses: vec![],
                },
            }),
        },
        sig_config: vec![0],
        signatures: vec![],
    });

    let sed = match dfs_serializer::to_string(&icp) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    let sig = match uw.sign_raw(&nk0.id, sed.as_bytes()) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    let sig_pref = Prefix::SigEd25519Sha512(sig);

    let signed_event = match icp {
        VersionedEventMessage::V0_0(ev) => VersionedEventMessage::V0_0(EventMessage {
            signatures: vec![sig_pref],
            ..ev
        }),
    };

    match serde_json::to_string(&WalletInceptionRep {
        id: uw.id.clone(),
        encrypted_wallet: export_wallet(uw, pass),
        inception_event: serialize_signed_message(&signed_event),
    }) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
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

pub fn change_id(encrypted_wallet: &str, id: &str, new_id: &str, pass: &str) -> String {
    let mut uw = match wallet_from(encrypted_wallet, id, &pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };
    uw.id = new_id.to_string();
    export_wallet(uw, pass)
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

pub fn add_content(encrypted_wallet: &str, id: &str, pass: &str, content: &str) -> String {
    let mut uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let content_entity: ContentEntity = match serde_json::from_str(content) {
        Ok(r) => r,
        Err(e) => return e.to_string(),
    };

    uw.import_content(content_entity);

    export_wallet(uw, pass)
}

pub fn set_key_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    controller: &str,
) -> String {
    let mut uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    uw.set_key_controller(key_ref, controller);

    export_wallet(uw, pass)
}

pub fn sign_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
    data: &str,
) -> String {
    let uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let data_bytes = match base64::decode_config(data, base64::URL_SAFE) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    let key_ref = match uw.get_key_by_controller(controller) {
        Some(c) => c.id,
        None => return "No Key Found".to_string(),
    };

    let sig = match uw.sign_raw(&key_ref, &data_bytes) {
        Ok(s) => s,
        Err(e) => return e.to_string(),
    };

    base64::encode_config(sig, base64::URL_SAFE)
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

pub fn verify(key_str: &str, key_type: &str, data: &str, sig: &str) -> bool {
    // use url safe or not?
    let key_bytes = match base64::decode_config(&key_str, base64::URL_SAFE) {
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

    match PublicKeyInfo::new(
        match KeyType::from_str(key_type) {
            Ok(t) => t,
            Err(_) => return false,
        },
        &key_bytes,
    )
    .verify(&data_bytes, &sig_bytes)
    {
        Ok(v) => v,
        Err(_) => false,
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

    let pk = match uw.get_key(&key_ref) {
        Some(pk) => pk,
        None => return "No key found".to_string(),
    };

    match serde_json::to_string(&pk) {
        Ok(s) => s,
        Err(e) => e.to_string(),
    }
}

pub fn get_key_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
) -> String {
    let uw = match wallet_from(encrypted_wallet, id, pass) {
        Ok(w) => w,
        Err(e) => return e.to_string(),
    };

    let pk = match uw.get_key_by_controller(controller) {
        Some(pk) => pk,
        None => return "No key found".to_string(),
    };

    match serde_json::to_string(&pk) {
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

    assert!(keys.len() > 16);

    Ok(())
}

#[test]
fn test2() -> Result<(), String> {
    let id = "my_did";
    let pass = "my_pass";
    let message = base64::encode_config("hello there", base64::URL_SAFE);

    let mut uw = UnlockedWallet::new(id);
    let k1 = uw
        .new_key(KeyType::EcdsaSecp256k1VerificationKey2019, None)
        .map_err(|_| "bad sig".to_string())?;

    let pk = match k1.content {
        Content::PublicKey(pk) => pk,
        _ => return Err("bad key".to_string()),
    };

    let pks = base64::encode_config(pk.public_key, base64::URL_SAFE);

    let lw = uw
        .lock(pass.as_bytes())
        .map_err(|_| "bad lock".to_string())?;

    let sig = sign(
        &base64::encode_config(lw.ciphertext, base64::URL_SAFE),
        id,
        pass,
        &k1.id,
        &message,
    );

    assert!(verify(
        &pks,
        "EcdsaSecp256k1VerificationKey2019",
        &message,
        &sig
    ));

    Ok(())
}

#[test]
fn test3() -> Result<(), String> {
    let kt = "EcdsaSecp256k1VerificationKey2019";
    let key = "Aw2CKxqxbAH5CJK5fo0LqnREgJQYYsFcAocCKX7TrUmp";
    let message = base64::encode_config("hello there".as_bytes(), base64::URL_SAFE);

    let sig =
        "dxolMmEAt56BaIgqTdAZ17QmmNcOA9wkmiVNwtVLr_0Ob3r0R2v9lqDMQxF8Pt--Jl9BDDyaxIsYsbAybZv3rw==";
    let wrong_sig =
        "rX1+vdS4/OelZZZZq/+2PJc70P2ZD2wu/eJINet5es9QVkDf7P70whQ84qvyF7Qp/wxVGbW/HWpTqjDCxrJDiA==";

    assert!(verify(key, kt, &message, sig));
    assert!(!verify(key, kt, &message, wrong_sig));

    Ok(())
}
