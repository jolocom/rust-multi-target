use thiserror::Error;
use base64::DecodeError;
use universal_wallet::Error as UwError;
use keri::error::Error as KeriError;
use keri::util::dfs_serializer::Error as DfsError;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DynError(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    WalletError(#[from] UwError),
    #[error(transparent)]
    KeriError(#[from] KeriError),
    #[error(transparent)]
    DfsError(#[from] DfsError),
    #[error(transparent)]
    StringFromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Decode64Error(#[from] DecodeError),
    #[error("{0}")]
    Generic(String),
}