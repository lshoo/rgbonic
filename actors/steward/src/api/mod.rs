mod get_ecdsa_key;
mod register_ecdsa_key;
mod update_ecdsa_key;

use candid::Principal;
use ic_cdk::export_candid;

use crate::{domain::UpdateKeyRequest, error::StewardError};

#[ic_cdk::query]
pub fn get_ecdsa_key() -> Result<String, StewardError> {
    let caller = ic_caller();
    get_ecdsa_key::serve(&caller)
}

#[ic_cdk::update]
pub fn register_ecdsa_key(key: String) -> Result<bool, StewardError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    register_ecdsa_key::serve(caller, key, updated_time)
}

#[ic_cdk::update]
pub fn update_ecdsa_key(req: UpdateKeyRequest) -> Result<bool, StewardError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    update_ecdsa_key::serve(caller, req.new_key, req.old_key, updated_time)
}

export_candid!();

fn ic_caller() -> Principal {
    ic_cdk::caller()
}

fn ic_time() -> u64 {
    ic_cdk::api::time()
}
