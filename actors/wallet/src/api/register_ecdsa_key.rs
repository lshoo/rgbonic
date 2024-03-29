use candid::Principal;

use crate::{context::METADATA, domain::Metadata, error::WalletError};

pub(super) fn serve(
    wallet: Principal,
    key: String,
    updated_time: u64,
) -> Result<bool, WalletError> {
    METADATA.with(|m| {
        let mut metadata = m.borrow_mut();

        if metadata.get().key.is_empty() {
            let md = metadata.get();
            metadata.set(Metadata { network: md.network, key, updated_time }).map_err(|e| WalletError::RegisterECDSAKeyError);
            Ok(true)
        } else {
            Err(WalletError::ECDSAKeyAlreadyExists(ic_cdk::id().to_string()))
        }
    })
}
