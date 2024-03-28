pub mod api;
pub mod bitcoin;
pub mod constants;
pub mod context;
pub mod domain;
pub mod rgb;

use candid::Principal;
use ic_cdk::export_candid;

#[ic_cdk::update]
fn greet(name: String) -> String {
    format!(
        "Hello, {}, management canister: {:?}",
        name,
        Principal::management_canister(),
    )
}

#[ic_cdk::init]
fn init() {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);
}

#[ic_cdk::update]
fn issue_rgb20() -> String {
    rgb::issue_rgb20()
}

pub fn ic_caller() -> Principal {
    ic_cdk::caller()
}

export_candid!();
