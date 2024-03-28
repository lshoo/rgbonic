use candid::{CandidType, Principal};

#[derive(Debug, thiserror::Error, CandidType)]
pub enum WalletError {
    #[error("Invalid transaction")]
    InvalidTransaction,

    #[error("{msg}")]
    CreateWalletError { msg: String },

    #[error("IO error")]
    DeserializeError,

    #[error("Call IC error: {0:?}")]
    ICCallError((ic_cdk::api::call::RejectionCode, String)),

    #[error("Invalid principal: {0:?} for a wallet")]
    InvalidPrincipal(Principal),

    #[error("Secp256k1 error: {0:?}")]
    Secp256k1Error(String),

    #[error("Bitcoin Address error: {0:?}")]
    BitcoinAddressError(String),

    #[error("{0:?} ECDSA key already exists")]
    ECDSAKeyAlreadyExists(String),

    #[error("{0:?} ECDSA key not found")]
    ECDSAKeyNotFound(String),

    #[error("Failed to update ECDSA key")]
    ECDSAKeyUpdateError,
}

impl From<(ic_cdk::api::call::RejectionCode, String)> for WalletError {
    fn from(e: (ic_cdk::api::call::RejectionCode, String)) -> Self {
        WalletError::ICCallError(e)
    }
}

impl From<bitcoin::secp256k1::Error> for WalletError {
    fn from(e: bitcoin::secp256k1::Error) -> Self {
        WalletError::Secp256k1Error(e.to_string())
    }
}

impl From<bitcoin::address::Error> for WalletError {
    fn from(e: bitcoin::address::Error) -> Self {
        WalletError::BitcoinAddressError(e.to_string())
    }
}
