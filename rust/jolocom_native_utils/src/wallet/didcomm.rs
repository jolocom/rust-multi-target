use std::convert::TryInto;

use didcomm_rs::crypto::CryptoAlgorithm;

use super::{
    UnlockedWallet,
    Error,
    wallet_from,
};

pub fn create_didcomm_message() -> String {
    UnlockedWallet::create_message()
}

pub fn create_jwe_didcomm_message(from: &str, to: &[&str], alg: &String) -> Result<String, Error> {
    let alg: CryptoAlgorithm = alg.try_into()
        .map_err(|_| Error::Generic(format!("failed to parse CryptoAlgorithm from {}", alg)))?;
    Ok(UnlockedWallet::create_jwe_message(from, to, alg))
}

pub fn seal_didcomm_message(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    message: &str,
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;
    Ok(uw.seal_encrypted(message)?)
}

pub fn seal_signed_didcomm_message(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    message: &str
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;
    Ok(uw.seal_signed(message)?)
}

pub fn receive_didcomm_message(
    encrypted_wallet: &str,
    id: &str,
    pass: &str,
    msg_bytes: &[u8],
) -> Result<String, Error> {
    let uw = wallet_from(encrypted_wallet, id, pass)?;
    Ok(uw.receive_message(msg_bytes)?
        .as_raw_json()
        .map_err(|e| Error::WalletError(e.into()))?)
}