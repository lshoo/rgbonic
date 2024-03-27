use ic_cdk::export_candid;

use super::ECDSA_KEY;

#[ic_cdk::query]
pub async fn get_ecdsa_key_name() -> String {
    ECDSA_KEY.with(|key| key.borrow().get().key.clone())
}

export_candid!();
