use candid::Principal;

use crate::{context::STATE, domain::Metadata, error::WalletError};

pub(super) fn serve(
    caller: &Principal,
    new_key: String,
    old_key: String,
    updated_time: u64,
) -> Result<bool, WalletError> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();

        match state.controllers.get(caller) {
            Some(_) => {
                let metadata = &mut state.metadata;

                let md = metadata.get();
                let current_key = &md.key;
                if current_key != &old_key {
                    return Err(WalletError::ECDSAKeyUpdateError);
                }

                metadata.set(Metadata { network: md.network, key: new_key, updated_time }).map_err(|_| WalletError::RegisterECDSAKeyError)?;
                Ok(true)
            }
            None => {
                Err(WalletError::UnAuthorized(caller.to_string()))
            }
        }
    })
}
