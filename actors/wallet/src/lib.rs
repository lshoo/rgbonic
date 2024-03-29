pub mod api;
pub mod bitcoin;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod rgb;

use crate::context::METADATA;
use crate::domain::{Metadata, UpdateKeyRequest};
use crate::error::WalletError;

use base::utils::format_network;
use candid::Principal;
use ic_cdk::export_candid;

#[ic_cdk::update]
fn greet(name: String) -> String {
    format!(
        "Hello, {}, management canister: {:?}",
        name,
        Principal::management_canister().to_string(),
    )
}



#[ic_cdk::init]
fn init(network: String) {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    METADATA.with(|m| {
        let mut metadata = m.borrow_mut();
        metadata
            .set(Metadata {
                network: format_network(&network),
                ..Default::default()
            })
            .expect("Failed to init network")
    });
}

#[ic_cdk::update]
fn issue_rgb20() -> String {
    rgb::issue_rgb20()
}

pub fn ic_caller() -> Principal {
    ic_cdk::caller()
}

export_candid!();
