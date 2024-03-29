use candid::CandidType;

#[derive(Debug, thiserror::Error, CandidType)]
pub enum WalletError {
    #[error("{0:?} ECDSA key already exists")]
    ECDSAKeyAlreadyExists(String),

    #[error("{0:?} ECDSA key not found")]
    ECDSAKeyNotFound(String),

    #[error("Failed to update ECDSA key")]
    ECDSAKeyUpdateError,

    #[error("Failed to register ECDSA key")]
    RegisterECDSAKeyError,

    #[error("Failed to init network")]
    NetworkAlreadyExists,
}
