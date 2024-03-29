
mod get_ecdsa_key;
mod register_ecdsa_key;
mod update_ecdsa_key;

use candid::Principal;
use ic_cdk::{query, update};

use crate::domain::{Metadata, UpdateKeyRequest};
use crate::context::METADATA;
use crate::error::WalletError;

#[update]
pub async fn get_wallet_address() -> String {
    crate::bitcoin::get_wallet_address().await
}

#[query]
pub fn get_ecdsa_key() -> Result<String, WalletError> {
    let caller = ic_caller();
    get_ecdsa_key::serve(&caller)
}

#[update]
pub fn register_ecdsa_key(key: String) -> Result<bool, WalletError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    register_ecdsa_key::serve(caller, key, updated_time)
}

#[update]
pub fn update_ecdsa_key(req: UpdateKeyRequest) -> Result<bool, WalletError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    update_ecdsa_key::serve(caller, req.new_key, req.old_key, updated_time)
}

#[query]
fn metadata() -> Metadata {
    METADATA.with(|m| m.borrow().get().clone())
}

fn ic_caller() -> Principal {
    ic_cdk::caller()
}

fn ic_time() -> u64 {
    ic_cdk::api::time()
}
