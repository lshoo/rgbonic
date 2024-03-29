use candid::Principal;

use crate::{context::STATE, domain::Metadata, error::WalletError};

pub(super) fn serve(
    caller: &Principal,
    key: String,
    updated_time: u64,
) -> Result<bool, WalletError> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        
        match state.controllers.get(caller) {
            Some(_) => {
                let metadata = &mut state.metadata;

                if metadata.get().key.is_empty() {
                    let md = metadata.get();
                    metadata.set(Metadata { network: md.network, key, updated_time }).map_err(|_| WalletError::RegisterECDSAKeyError)?;
                    Ok(true)
                } else {
                    Err(WalletError::ECDSAKeyAlreadyExists(ic_cdk::id().to_string()))
                }
            }
            None => {
                Err(WalletError::UnAuthorized(caller.to_string()))
            }
        }
    })
}
