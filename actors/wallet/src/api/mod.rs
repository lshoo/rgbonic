mod get_ecdsa_key;
mod register_ecdsa_key;
mod update_ecdsa_key;

use candid::Principal;
use ic_cdk::{query, update};

use crate::context::{State, STATE};
use crate::domain::{Metadata, UpdateKeyRequest};
use crate::error::WalletError;

#[update]
pub async fn get_wallet_address() -> String {
    let caller = ic_caller();

    crate::bitcoin::get_wallet_address(&caller).await
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

    register_ecdsa_key::serve(&caller, key, updated_time)
}

#[update]
pub fn update_ecdsa_key(req: UpdateKeyRequest) -> Result<bool, WalletError> {
    let caller = ic_caller();
    let updated_time = ic_time();

    update_ecdsa_key::serve(&caller, req.new_key, req.old_key, updated_time)
}

#[query]
fn metadata() -> Metadata {
    STATE.with(|m| m.borrow().metadata.get().clone())
}

#[query]
fn controller() -> Result<Vec<Principal>, WalletError> {
    let caller = ic_caller();

    STATE.with(|s| {
        let state = s.borrow();

        match state.controllers.get(&caller) {
            Some(_) => Ok(state.controllers.iter().map(|(k, _)| k).collect()),
            None => Err(WalletError::UnAuthorized(caller.to_string())),
        }
    })
}

fn ic_caller() -> Principal {
    ic_cdk::caller()
}

fn ic_time() -> u64 {
    ic_cdk::api::time()
}

fn validate_controller<F, T>(state: &State, caller: &Principal, f: F) -> Result<T, WalletError>
where
    F: FnOnce(&State) -> Result<T, WalletError>,
{
    match state.controllers.get(caller) {
        Some(_) => f(state),
        None => Err(WalletError::UnAuthorized(caller.to_string())),
    }
}

fn validate_controller_mut<F, T>(
    state: &mut State,
    caller: &Principal,
    mut f: F,
) -> Result<T, WalletError>
where
    F: FnMut(&mut State) -> Result<T, WalletError>,
{
    match state.controllers.get(caller) {
        Some(_) => f(state),
        None => Err(WalletError::UnAuthorized(caller.to_string())),
    }
}
