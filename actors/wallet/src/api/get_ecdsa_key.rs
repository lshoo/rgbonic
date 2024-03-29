use candid::Principal;

use crate::{context::STATE, error::WalletError};

use super::validate_controller;

/// Only controller can call this
pub(crate) fn serve(caller: &Principal) -> Result<String, WalletError> {
    STATE.with(|s| {
        let state = s.borrow();

        // match state.controllers.get(caller) {
        //     Some(_) => {
        //         let key = &state.metadata.get().key;
        //         if key.is_empty() {
        //             Err(WalletError::ECDSAKeyNotFound(ic_cdk::id().to_string()))
        //         } else {
        //             Ok(key.to_string())
        //         }
        //     }
        //     None => {
        //         Err(WalletError::UnAuthorized(caller.to_string()))
        //     }
        // }

        validate_controller(&state, caller, |s| {
            let key = &s.metadata.get().key;
            if key.is_empty() {
                Err(WalletError::ECDSAKeyNotFound(ic_cdk::id().to_string()))
            } else {
                Ok(key.to_string())
            }
        })
    })
}
