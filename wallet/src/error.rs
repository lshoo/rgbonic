#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("Invalid transaction")]
    InvalidTransaction,

    #[error("{msg}")]
    CreateWalletError { msg: String },

    #[error("IO error")]
    DeserializeError,

    #[error("Call IC error: {0:?}")]
    ICCallError((ic_cdk::api::call::RejectionCode, String)),
}

impl From<(ic_cdk::api::call::RejectionCode, String)> for WalletError {
    fn from(e: (ic_cdk::api::call::RejectionCode, String)) -> Self {
        WalletError::ICCallError(e)
    }
}
