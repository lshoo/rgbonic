use candid::Principal;

use crate::{context::METADATA, domain::Metadata, error::WalletError};

pub(super) fn serve(
    wallet: Principal,
    new_key: String,
    old_key: String,
    updated_time: u64,
) -> Result<bool, WalletError> {
    METADATA.with(|m| {
        let mut metadata = m.borrow_mut();

        let md = metadata.get();
        let current_key = md.key;
        if current_key != old_key {
            return Err(WalletError::ECDSAKeyUpdateError);
        }

        metadata.set(Metadata { network: md.network, key: new_key, updated_time }).map_err(|e| WalletError::RegisterECDSAKeyError);
        Ok(true)
        
    })
}
