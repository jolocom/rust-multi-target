use base64;
use core::str::FromStr;
use keriox::{
    derivation::blake2b_256_digest,
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

pub fn get_random_b64(len: usize) -> Result<String, String> {
    Ok(base64::encode_config(get_random(len)?, base64::URL_SAFE))
}

pub fn wallet_from(encrypted_wallet: &str, id: &str, pass: &str) -> Result<UnlockedWallet, String> {
    let ew = match base64::decode_config(encrypted_wallet, base64::URL_SAFE) {
        Ok(w) => w,
        Err(e) => return Err(e.to_string()),
    };
    let lw = LockedWallet::new(id, ew);

    lw.unlock(pass.as_bytes())
}

pub fn new_wallet(id: &str, pass: &str) -> Result<String, String> {
    export_wallet(UnlockedWallet::new(&id), &pass)
}

pub fn incept_wallet(encrypted_wallet: &str, id: &str, pass: &str) -> Result<String, String> {
    let mut uw = wallet_from(encrypted_wallet, id, pass)?;

    let nk0 = uw.new_key(KeyType::Ed25519VerificationKey2018, None)?;

    let pref0 = match &nk0.content {
        Content::PublicKey(pk) => Prefix::PubKeyEd25519(pk.public_key.clone()),
        _ => return Err("Wrong Content Type".to_string()),
    };

    uw.id = ["did:un", &pref0.to_string()].join(":");
    uw.set_key_controller(&nk0.id, &[uw.id.clone(), pref0.to_string()].join("#"));

    let nk1 = uw.new_key(KeyType::Ed25519VerificationKey2018, None)?;

    let pref1 = match &nk1.content {
        Content::PublicKey(pk) => Prefix::Blake2B256(blake2b_256_digest(pk.public_key.as_ref())),
        _ => return Err("Wrong Content Type".to_string()),
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

    let sed = dfs_serializer::to_string(&icp).map_err(|e| e.to_string())?;

    let sig = uw.sign_raw(&nk0.id, sed.as_bytes())?;

    let sig_pref = Prefix::SigEd25519Sha512(sig);

    let signed_event = match icp {
        VersionedEventMessage::V0_0(ev) => VersionedEventMessage::V0_0(EventMessage {
            signatures: vec![sig_pref],
            ..ev
        }),
    };

    serde_json::to_string(&WalletInceptionRep {
        id: uw.id.clone(),
        encrypted_wallet: export_wallet(uw, pass)?,
        inception_event: serialize_signed_message(&signed_event),
    })
    .map_err(|e| e.to_string())
}

pub fn export_wallet(uw: UnlockedWallet, pass: &str) -> Result<String, String> {
    Ok(base64::encode_config(
        uw.lock(pass.as_bytes())?.ciphertext,
        base64::URL_SAFE,
    ))
}

pub fn change_pass(
    encrypted_wallet: &str,
    id: &str,
    old_pass: &str,
    new_pass: &str,
) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, &old_pass)?;
    export_wallet(uw, new_pass)
}

pub fn change_id(
    encrypted_wallet: &str,
    id: &str,
    new_id: &str,
    pass: &str,
) -> Result<String, String> {
    let mut uw = wallet_from(encrypted_wallet, id, &pass)?;
    uw.id = new_id.to_string();
    export_wallet(uw, pass)
}

pub fn new_key(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_type: &str,
    controller: Option<Vec<String>>,
) -> Result<String, String> {
    let mut uw = wallet_from(encrypted_wallet, id, pass)?;

    let nkt = KeyType::from_str(key_type)?;

    let key = uw.new_key(nkt, controller)?;

    serde_json::to_string(&AddKeyResultRep {
        new_encrypted_state: export_wallet(uw, pass)?,
        new_key: key,
    })
    .map_err(|e| e.to_string())
}

pub fn add_content(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    content: &str,
) -> Result<String, String> {
    let mut uw = wallet_from(encrypted_wallet, id, pass)?;

    let content_entity: ContentEntity = serde_json::from_str(content).map_err(|e| e.to_string())?;

    uw.import_content(content_entity);

    export_wallet(uw, pass)
}

pub fn set_key_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    controller: &str,
) -> Result<String, String> {
    let mut uw = wallet_from(encrypted_wallet, id, pass)?;

    uw.set_key_controller(key_ref, controller);

    export_wallet(uw, pass)
}

pub fn sign_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
    data: &str,
) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let key_ref = match uw.get_key_by_controller(controller) {
        Some(c) => c.id,
        None => return Err("No Key Found".to_string()),
    };

    let sig_bytes = uw.sign_raw(&key_ref, &data_bytes)?;

    Ok(base64::encode_config(sig_bytes, base64::URL_SAFE))
}

pub fn sign(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    data: &str,
) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let sig = uw.sign_raw(key_ref, &data_bytes)?;

    Ok(base64::encode_config(sig, base64::URL_SAFE))
}

pub fn verify(key_str: &str, key_type: &str, data: &str, sig: &str) -> Result<bool, String> {
    // use url safe or not?
    let key_bytes = base64::decode_config(key_str, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let sig_bytes = base64::decode_config(sig, base64::URL_SAFE).map_err(|e| e.to_string())?;

    PublicKeyInfo::new(KeyType::from_str(key_type)?, &key_bytes).verify(&data_bytes, &sig_bytes)
}

pub fn decrypt_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
    data: &str,
    aad: &str,
) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let aad_bytes = base64::decode_config(aad, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let key_ref = match uw.get_key_by_controller(controller) {
        Some(c) => c.id,
        None => return Err("No Key Found".to_string()),
    };

    let decrypted = uw.decrypt(&key_ref, &data_bytes, &aad_bytes)?;

    Ok(base64::encode_config(decrypted, base64::URL_SAFE))
}

pub fn decrypt(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    data: &str,
    aad: &str,
) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let aad_bytes = base64::decode_config(aad, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let decrypted = uw.decrypt(key_ref, &data_bytes, &aad_bytes)?;

    Ok(base64::encode_config(decrypted, base64::URL_SAFE))
}

pub fn encrypt(key: &str, key_type: &str, data: &str, aad: &str) -> Result<String, String> {
    let key_bytes = base64::decode_config(key, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE).map_err(|e| e.to_string())?;

    let aad_bytes = base64::decode_config(aad, base64::URL_SAFE).map_err(|e| e.to_string())?;

    match PublicKeyInfo::new(KeyType::from_str(key_type)?, &key_bytes)
        .encrypt(&data_bytes, &aad_bytes)
    {
        Ok(v) => Ok(base64::encode_config(v, base64::URL_SAFE)),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_keys(encrypted_wallet: &str, id: &str, pass: &str) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    match serde_json::to_string(&uw.get_keys()) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_key(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let pk = match uw.get_key(&key_ref) {
        Some(pk) => pk,
        None => return Err("No key found".to_string()),
    };

    match serde_json::to_string(&pk) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_key_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
) -> Result<String, String> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let pk = match uw.get_key_by_controller(controller) {
        Some(pk) => pk,
        None => return Err("No key found".to_string()),
    };

    match serde_json::to_string(&pk) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
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
