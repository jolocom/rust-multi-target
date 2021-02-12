use crate::error::Error;
use base64;
use core::str::FromStr;
use keri::{
    derivation::{
        basic::{Basic, PublicKey},
        self_addressing::SelfAddressing,
        self_signing::SelfSigning,
    },
    event::{
        event_data::inception::InceptionEvent,
        sections::{nxt_commitment, KeyConfig},
    },
    event_message::serialization_info::SerializationFormats,
    prefix::{AttachedSignaturePrefix, Prefix},
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert::TryInto;
use universal_wallet::{get_random, prelude::*, Error as UwError};

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

pub fn get_random_b64(len: usize) -> Result<String, Error> {
    Ok(base64::encode_config(get_random(len)?, base64::URL_SAFE))
}

pub fn wallet_from(encrypted_wallet: &str, id: &str, pass: &str) -> Result<UnlockedWallet, Error> {
    let ew = base64::decode_config(encrypted_wallet, base64::URL_SAFE)?;
    //  {
    //     Ok(w) => w,
    //     Err(e) => return Err(e.to_string()),
    // };
    let lw = LockedWallet::new(id, ew);

    Ok(lw.unlock(pass.as_bytes())?)
}

pub fn new_wallet(id: &str, pass: &str) -> Result<String, Error> {
    export_wallet(UnlockedWallet::new(&id), &pass)
}

pub fn incept_populated_wallet(
    signing_enc_keys_str: &str,
    pre_rotated_keys: &str,
    pass: &str,
) -> Result<String, Error> {
    let mut uw = UnlockedWallet::new("");

    let signing_enc_keys: Vec<&str> =
        serde_json::from_str(signing_enc_keys_str).map_err(|e| UwError::Serde(e))?;
    let pre_rotated_keys: Vec<&str> =
        serde_json::from_str(pre_rotated_keys).map_err(|e| UwError::Serde(e))?;

    let sig_key_0 = KeyPair::new(
        KeyType::Ed25519VerificationKey2018,
        &base64::decode_config(signing_enc_keys[0], base64::URL_SAFE).unwrap(),
    )
    .unwrap();

    let sig_key_1 = KeyPair::new(
        KeyType::Ed25519VerificationKey2018,
        &base64::decode_config(pre_rotated_keys[0], base64::URL_SAFE).unwrap(),
    )
    .unwrap();

    let enc_key_0 = KeyPair::new(
        KeyType::X25519KeyAgreementKey2019,
        &base64::decode_config(signing_enc_keys[1], base64::URL_SAFE).unwrap(),
    )
    .unwrap();

    let enc_key_1 = KeyPair::new(
        KeyType::X25519KeyAgreementKey2019,
        &base64::decode_config(pre_rotated_keys[1], base64::URL_SAFE).unwrap(),
    )
    .unwrap();

    let sig_pref_0 = Basic::Ed25519.derive(PublicKey {
        0: sig_key_0.public_key.public_key.clone(),
    });
    let sig_pref_1 = Basic::Ed25519.derive(PublicKey {
        0: sig_key_1.public_key.public_key.clone(),
    });
    let enc_pref_0 = Basic::X25519.derive(PublicKey {
        0: enc_key_0.public_key.public_key.clone(),
    });
    let enc_pref_1 = Basic::X25519.derive(PublicKey {
        0: enc_key_1.public_key.public_key.clone(),
    });

    // next key set pre-commitment
    let nexter_pref = nxt_commitment(
        1,
        &[sig_pref_1.clone(), enc_pref_1.clone()],
        SelfAddressing::Blake3_256,
    );

    let icp = InceptionEvent::new(
        KeyConfig::new(
            vec![sig_pref_0.clone(), enc_pref_0.clone()],
            Some(nexter_pref),
            Some(1),
        ),
        None,
        None,
    )
    .incept_self_addressing(SelfAddressing::Blake3_256, SerializationFormats::JSON)?;

    uw.id = ["did:keri", &icp.event.prefix.to_str()].join(":");
    let sig_0_controller = vec![[uw.id.clone(), sig_pref_0.to_str()].join("#").to_string()];
    let key_id = uw
        .import_content(&Content::KeyPair(
            sig_key_0.set_controller(sig_0_controller),
        ))
        .unwrap()
        .id;

    let sig_1_controller = vec![[uw.id.clone(), sig_pref_1.to_str()].join("#").to_string()];
    uw.import_content(&Content::KeyPair(
        sig_key_1.set_controller(sig_1_controller),
    ));

    let enc_key_0_controller = vec![[uw.id.clone(), enc_pref_0.to_str()].join("#").to_string()];
    uw.import_content(&Content::KeyPair(
        enc_key_0.set_controller(enc_key_0_controller),
    ));

    let enc_key_1_controller = vec![[uw.id.clone(), enc_pref_1.to_str()].join("#").to_string()];
    uw.import_content(&Content::KeyPair(
        enc_key_1.set_controller(enc_key_1_controller),
    ));

    // serialised
    let serialized = icp.serialize()?;

    // sign
    let sig_pref = AttachedSignaturePrefix::new(
        SelfSigning::Ed25519Sha512,
        uw.sign_raw(&key_id, &serialized)?,
        0,
    );

    let signed_event = icp.sign(vec![sig_pref]);

    Ok(serde_json::to_string(&WalletInceptionRep {
        id: uw.id.clone(),
        encrypted_wallet: export_wallet(uw, pass)?,
        // the serialized json event is guarenteed to be valid utf-8
        inception_event: String::from_utf8(signed_event.serialize()?)?,
    })
    .map_err(|e| UwError::Serde(e))?)
}

pub fn incept_wallet(encrypted_wallet: &str, id: &str, pass: &str) -> Result<String, Error> {
    let mut uw = wallet_from(encrypted_wallet, id, pass)?;

    let sig_key_0 = uw.new_key(KeyType::Ed25519VerificationKey2018, None)?;
    let enc_key_0 = uw.new_key(KeyType::X25519KeyAgreementKey2019, None)?;

    let sig_key_1 = uw.new_key(KeyType::Ed25519VerificationKey2018, None)?;
    let enc_key_1 = uw.new_key(KeyType::X25519KeyAgreementKey2019, None)?;

    let sig_pref_0 = match &sig_key_0.content {
        Content::PublicKey(pk) => Basic::Ed25519.derive(PublicKey {
            0: pk.public_key.clone(),
        }),
        _ => return Err(Error::Generic("Wrong Content Type".to_string())),
    };
    let enc_pref_0 = match &enc_key_0.content {
        Content::PublicKey(pk) => Basic::X25519.derive(PublicKey {
            0: pk.public_key.clone(),
        }),
        _ => return Err(Error::Generic("Wrong Content Type".to_string())),
    };

    let sig_pref_1 = match &sig_key_1.content {
        Content::PublicKey(pk) => Basic::Ed25519.derive(PublicKey {
            0: pk.public_key.clone(),
        }),
        _ => return Err(Error::Generic("Wrong Content Type".to_string())),
    };
    let enc_pref_1 = match &enc_key_1.content {
        Content::PublicKey(pk) => Basic::X25519.derive(PublicKey {
            0: pk.public_key.clone(),
        }),
        _ => return Err(Error::Generic("Wrong Content Type".to_string())),
    };

    // TODO: is this needed here?
    // let nexter_pref = SelfAddressing::Blake3_256.derive(
    //     [sig_pref_1.to_str(), enc_pref_1.to_str()]
    //         .join("")
    //         .as_bytes(),
    // );

    // next key set pre-commitment
    let nexter_pref = nxt_commitment(1, &[sig_pref_1, enc_pref_1], SelfAddressing::Blake3_256);

    let icp = InceptionEvent::new(
        KeyConfig::new(
            vec![sig_pref_0.clone(), enc_pref_0.clone()],
            Some(nexter_pref),
            Some(1),
        ),
        None,
        None,
    )
    .incept_self_addressing(SelfAddressing::Blake3_256, SerializationFormats::JSON)?;

    uw.id = ["did:keri", &icp.event.prefix.to_str()].join(":");
    uw.set_key_controller(
        &sig_key_0.id,
        &[uw.id.clone(), sig_pref_0.to_str()].join("#"),
    );
    uw.set_key_controller(
        &enc_key_0.id,
        &[uw.id.clone(), enc_pref_0.to_str()].join("#"),
    );

    let serialized = icp.serialize()?;

    let sig_pref = AttachedSignaturePrefix::new(
        SelfSigning::Ed25519Sha512,
        uw.sign_raw(&sig_key_0.id, &serialized)?,
        0,
    );

    let signed_event = icp.sign(vec![sig_pref]);

    Ok(serde_json::to_string(&WalletInceptionRep {
        id: uw.id.clone(),
        encrypted_wallet: export_wallet(uw, pass)?,
        inception_event: String::from_utf8(signed_event.serialize()?)?,
    })
    .map_err(|e| UwError::Serde(e))?)
}

pub fn export_wallet(uw: UnlockedWallet, pass: &str) -> Result<String, Error> {
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
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, &old_pass)?;
    export_wallet(uw, new_pass)
}

pub fn change_id(
    encrypted_wallet: &str,
    id: &str,
    new_id: &str,
    pass: &str,
) -> Result<String, Error> {
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
) -> Result<String, Error> {
    let mut uw = wallet_from(encrypted_wallet, id, pass)?;

    let nkt = KeyType::from_str(key_type)?;

    let key = uw.new_key(nkt, controller)?;

    Ok(serde_json::to_string(&AddKeyResultRep {
        new_encrypted_state: export_wallet(uw, pass)?,
        new_key: key,
    })
    .map_err(|e| UwError::Serde(e))?)
}

pub fn add_content(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    content: &str,
) -> Result<String, Error> {
    let mut uw = wallet_from(encrypted_wallet, id, pass)?;

    let content_entity: Content = serde_json::from_str(content).map_err(|e| UwError::Serde(e))?;

    uw.import_content(&content_entity);

    export_wallet(uw, pass)
}

pub fn set_key_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    controller: &str,
) -> Result<String, Error> {
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
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE)?;

    let key_ref = match uw.get_key_by_controller(controller) {
        Some(c) => c.id,
        None => return Err(Error::Generic("No Key Found".to_string())),
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
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE)?;

    let sig = uw.sign_raw(key_ref, &data_bytes)?;

    Ok(base64::encode_config(sig, base64::URL_SAFE))
}

// pub fn verify(key_str: &str, key_type: &str, data: &str, sig: &str) -> Result<bool, Error> {
//     // use url safe or not?
//     let key_bytes = base64::decode_config(key_str, base64::URL_SAFE)?;

//     let data_bytes = base64::decode_config(data, base64::URL_SAFE)?;

//     let sig_bytes = base64::decode_config(sig, base64::URL_SAFE)?;

//     let pk = PublicKeyInfo::new(key_type.try_into()?, &key_bytes);

//     let verification_result = pk.verify(&data_bytes, &sig_bytes);

//     Ok(verification_result?)
// }

pub fn verify(key_str: &str, key_type: &str, data: &str, signature: &str) -> Result<bool, Error> {
    Ok(
        PublicKeyInfo::new(key_type.try_into()?, &decode_base64_url_safe(key_str)?).verify(
            &decode_base64_url_safe(data)?,
            &decode_base64_url_safe(signature)?,
        )?,
    )
}

fn decode_base64_url_safe(encoded: &str) -> Result<Vec<u8>, Error> {
    Ok(base64::decode_config(encoded, base64::URL_SAFE)?)
}

pub fn decrypt_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
    data: &str,
    aad: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE)?;

    let aad_bytes = base64::decode_config(aad, base64::URL_SAFE)?;

    let key_ref = match uw.get_key_by_controller(controller) {
        Some(c) => c.id,
        None => return Err(Error::Generic("No Key Found".to_string())),
    };

    let decrypted = uw.decrypt(&key_ref, &data_bytes, Some(&aad_bytes))?;

    Ok(base64::encode_config(decrypted, base64::URL_SAFE))
}

pub fn decrypt(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    data: &str,
    aad: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE)?;

    let aad_bytes = base64::decode_config(aad, base64::URL_SAFE)?;

    let decrypted = uw.decrypt(key_ref, &data_bytes, Some(&aad_bytes))?;

    Ok(base64::encode_config(decrypted, base64::URL_SAFE))
}

pub fn encrypt(key: &str, key_type: &str, data: &str, aad: &str) -> Result<String, Error> {
    let key_bytes = base64::decode_config(key, base64::URL_SAFE)?;

    let data_bytes = base64::decode_config(data, base64::URL_SAFE)?;

    let aad_bytes = base64::decode_config(aad, base64::URL_SAFE)?;

    let pki = PublicKeyInfo::new(KeyType::from_str(key_type)?, &key_bytes)
        .encrypt(&data_bytes, Some(&aad_bytes))?;
    Ok(base64::encode_config(pki, base64::URL_SAFE))
}

pub fn get_keys(encrypted_wallet: &str, id: &str, pass: &str) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    Ok(serde_json::to_string(&uw.get_keys()).map_err(|e| UwError::Serde(e))?)
}

pub fn get_key(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let pk = match uw.get_key(&key_ref) {
        Some(pk) => pk,
        None => return Err(Error::Generic("No key found".to_string())),
    };

    Ok(serde_json::to_string(&pk).map_err(|e| UwError::Serde(e))?)
}

pub fn get_key_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let pk = match uw.get_key_by_controller(controller) {
        Some(pk) => pk,
        None => return Err(Error::Generic("No key found".to_string())),
    };

    Ok(serde_json::to_string(&pk).map_err(|e| UwError::Serde(e))?)
}

pub fn ecdh_get_shared_secret(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_ref: &str,
    pub_key: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let pub_key_bytes = base64::decode_config(pub_key, base64::URL_SAFE)?;

    if pub_key_bytes.len() != 32 {
        return Err(Error::Generic("Invalid X25519 Pub Key Size".to_owned()));
    }

    let shared_secret = uw.ecdh_key_agreement(key_ref, &pub_key_bytes)?;

    Ok(base64::encode_config(shared_secret, base64::URL_SAFE))
}

pub fn ecdh_get_shared_secret_by_controller(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    controller: &str,
    pub_key: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;

    let key_ref = match uw.get_key_by_controller(controller) {
        Some(c) => c.id,
        None => return Err(Error::Generic("No Key Found".to_string())),
    };

    let pub_key_bytes = base64::decode_config(pub_key, base64::URL_SAFE)?;

    if pub_key_bytes.len() != 32 {
        return Err(Error::Generic("Invalid X25519 Pub Key Size".to_owned()));
    }

    let shared_secret = uw.ecdh_key_agreement(&key_ref, &pub_key_bytes)?;

    Ok(base64::encode_config(shared_secret, base64::URL_SAFE))
}

pub fn create_didcomm_message() -> String {
    UnlockedWallet::create_message()
}

pub fn seal_didcomm_message(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_id: &str,
    message: &str,
    header: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;
    Ok(uw.seal_encrypted(key_id, message, header)?)
}

pub fn seal_signed_didcomm_message(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    key_id: &str,
    sign_key_id: &str,
    message: &str,
    header: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;
    Ok(uw.seal_signed(key_id, sign_key_id, message, header)?)
}

pub fn receive_didcomm_message(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    msg_bytes: &[u8],
    sender_public_key: &[u8],
    verifying_key: Option<&[u8]>,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;
    Ok(uw
        .receive_message(msg_bytes, sender_public_key, verifying_key)?
        .as_raw_json()
        .map_err(|e| Error::WalletError(e.into()))?)
}

#[test]
fn test_create() -> Result<(), Error> {
    let id = "my_did".to_string();
    let p = "my_password".to_string();

    let ew = new_wallet(&id, &p)?;

    let res: AddKeyResultRep = serde_json::from_str(&new_key(
        &ew,
        &id,
        &p,
        "EcdsaSecp256k1VerificationKey2019",
        None,
    )?)
    .map_err(|e| UwError::Serde(e))?;

    let keys = get_keys(&res.new_encrypted_state, &id, &p)?;
    assert!(keys.len() > 16);

    Ok(())
}

#[test]
fn test_incept() -> Result<(), Error> {
    // use crate::{validate_events_str, did_document::{KeyTypes, DIDDocument}};

    // let id = "my_did";
    // let p = "my_password";

    // let ew = new_wallet(id, p)?;

    // let res_str: WalletInceptionRep =
    //     serde_json::from_str(&incept_wallet(&ew, id, p)?).map_err(|e| UwError::Serde(e))?;

    // let nid: String = res_str.id.clone();

    // let uw = LockedWallet::new(
    //     &nid,
    //     base64::decode_config(&res_str.encrypted_wallet, base64::URL_SAFE)?
    //     ).unlock(p.as_bytes())?;

    // assert_eq!(uw.get_keys().len(), 4);

    // let kel_bytes = res_str.inception_event.as_bytes();

    // let ddo_str = validate_events_str(&kel_bytes, "jun")?;

    // let ddo: DIDDocument = serde_json::from_str(&ddo_str).map_err(|e| UwError::Serde(e))?;

    // assert_eq!(ddo.verification_methods.len(), 2);
    // assert_eq!(
    //     ddo.verification_methods[0].key_type,
    //     KeyTypes::Ed25519VerificationKey2018
    // );
    // assert_eq!(
    //     ddo.verification_methods[1].key_type,
    //     KeyTypes::X25519KeyAgreementKey2019
    // );

    Ok(())
}

#[test]
fn test_rt_sign() -> Result<(), Error> {
    let id = "my_did";
    let pass = "my_pass";
    let message = base64::encode_config("hello there", base64::URL_SAFE);

    let mut uw = UnlockedWallet::new(id);
    let k1 = uw.new_key(KeyType::EcdsaSecp256k1VerificationKey2019, None)?;
    // .map_err(|_| "bad sig".to_string())?; // TODO: should this be transparent or use this string?

    let pk = match k1.content {
        Content::PublicKey(pk) => pk,
        _ => return Err(Error::Generic("bad key".to_string())),
    };

    let pks = base64::encode_config(pk.public_key, base64::URL_SAFE);

    let lw = uw.lock(pass.as_bytes())?;

    let sig = sign(
        &base64::encode_config(lw.ciphertext, base64::URL_SAFE),
        id,
        pass,
        &k1.id,
        &message,
    )?;

    assert!(verify(
        &pks,
        "EcdsaSecp256k1VerificationKey2019",
        &message,
        &sig
    )?);

    Ok(())
}

#[test]
fn test_sign() -> Result<(), Error> {
    let kt = "EcdsaSecp256k1VerificationKey2019";
    let key = "Aw2CKxqxbAH5CJK5fo0LqnREgJQYYsFcAocCKX7TrUmp";
    let message = base64::encode_config("hello there".as_bytes(), base64::URL_SAFE);

    let sig =
        "dxolMmEAt56BaIgqTdAZ17QmmNcOA9wkmiVNwtVLr_0Ob3r0R2v9lqDMQxF8Pt--Jl9BDDyaxIsYsbAybZv3rw==";
    let wrong_sig =
        "dxolAAAAt56BaIgqTdAZ17QmmNcOA9wkmiVNwtVLr_0Ob3r0R2v9lqDMQxF8Pt--Jl9BDDyaxIsYsbAybZv3rw==";

    assert!(verify(key, kt, &message, sig)?);
    assert!(!verify(key, kt, &message, wrong_sig)?);

    Ok(())
}

#[test]
fn test_key_type() -> Result<(), Error> {
    let kt = KeyType::from_str("EcdsaSecp256k1VerificationKey2019");
    assert!(&kt.is_ok());
    let kt = kt?;
    let expected = KeyType::EcdsaSecp256k1VerificationKey2019;
    assert_eq!(kt, expected);
    let pki_1 = PublicKeyInfo::new(
        KeyType::from_str("EcdsaSecp256k1VerificationKey2019")?,
        b"nokey",
    );
    let pki_2 = PublicKeyInfo::new("EcdsaSecp256k1VerificationKey2019".try_into()?, b"nokey");
    let pki_3 = PublicKeyInfo::new(kt, b"nokey");
    assert_eq!(pki_1.key_type, KeyType::EcdsaSecp256k1VerificationKey2019);
    assert_eq!(pki_2.key_type, KeyType::EcdsaSecp256k1VerificationKey2019);
    assert_eq!(pki_3.key_type, KeyType::EcdsaSecp256k1VerificationKey2019);
    Ok(())
}

#[test]
fn test_incept_from_keys() -> Result<(), Error> {
    // use crate::validate_events_str;

    // let pass = "secret";
    // let sign_enc_keys = "[\"JsdnEtidkG5mctr6YUxC5cscqsjGVo5NJJMIfbUfDTY\",\"JsdnEtidkG5mctr6YUxC5cscqsjGVo5NJJMIfbUfDTY\"]";
    // let pre_rot_sign_enc_keys = sign_enc_keys.clone();
    // let res_str: WalletInceptionRep = serde_json::from_str(&incept_populated_wallet(
    //     sign_enc_keys,
    //     pre_rot_sign_enc_keys,
    //     pass,
    // )?)
    // .map_err(|e| UwError::Serde(e))?;

    // let wallet = LockedWallet::new(
    //     &res_str.id,
    //     base64::decode_config(&res_str.encrypted_wallet, base64::URL_SAFE).unwrap(),
    // );

    // let uw = wallet.unlock(pass.as_bytes()).unwrap();
    // let kel_bytes = &res_str.inception_event.as_bytes();
    // let ddo_str = validate_events_str(kel_bytes, "jun")?;
    // let expected = "{\
    //     \"@context\":\"https://www.w3.org/ns/did/v1\",\"id\":\"did:jun:E5Yfn7S_a94Dcme771a0_CikVD2jcWOfGC0cXPZwUNO4\",\
    //     \"verificationMethod\":[{\"id\":\"#DwiR4cFNqGS0ULQVqvvymjWGTFY58GlnBZUyVTsyv-JQ\",\"type\":\"Ed25519VerificationKey2018\",\
    //     \"controller\":\"did:jun:E5Yfn7S_a94Dcme771a0_CikVD2jcWOfGC0cXPZwUNO4\",\"publicKeyBase64\":\
    //     \"wiR4cFNqGS0ULQVqvvymjWGTFY58GlnBZUyVTsyv-JQ=\"},{\"id\":\"#CZTkGQSfHcFmTRGoLESSby0wGup4XBDP3IkJ6tYpQ_0w\",\"type\":\
    //     \"X25519KeyAgreementKey2019\",\"controller\":\"did:jun:E5Yfn7S_a94Dcme771a0_CikVD2jcWOfGC0cXPZwUNO4\",\"publicKeyBase64\":\
    //     \"ZTkGQSfHcFmTRGoLESSby0wGup4XBDP3IkJ6tYpQ_0w=\"}]}";
    // assert_eq!(ddo_str, expected);

    // assert_eq!(uw.get_keys().len(), 4);
    Ok(())
}

#[test]
fn test_add_content() -> Result<(), Error> {
    let id = "id";
    let pass = "pass";
    let content_1 = r#"{"type":["TestEntropy"],"value":"Gf6rvA=="}"#;
    let content_2 = r#"{"controller":["ecdh_key"],"type":"X25519KeyAgreementKey2019","publicKeyHex":"8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a","private_key":"77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a"}"#;

    let ew0 = export_wallet(UnlockedWallet::new(id), pass)?;
    let ew1 = add_content(&ew0, id, pass, content_1)?;
    let ew2 = add_content(&ew1, id, pass, content_2)?;

    assert!(ew0.len() < ew1.len());
    assert!(ew1.len() < ew2.len());

    LockedWallet::new(id, base64::decode_config(&ew2, base64::URL_SAFE)?)
        .unlock(pass.as_bytes())?;

    Ok(())
}
