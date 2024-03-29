use candid::Principal;

use crate::{context::METADATA, error::WalletError};

pub(crate) fn serve(caller: &Principal) -> Result<String, WalletError> {
    METADATA.with(|m| {
        let key = m.borrow().get().key;
        
        if key.is_empty() {
            Err(WalletError::ECDSAKeyNotFound(ic_cdk::id().to_string()))
        } else {
            Ok(key)
        }
    })
}
