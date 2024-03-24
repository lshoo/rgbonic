#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Invalid transaction")]
    InvalidTransaction,

    #[error("{msg}")]
    CreateWalletError { msg: String },

    #[error("IO error")]
    DeserializeError,
}
