pub mod api;
pub mod bitcoin;
pub mod constants;
pub mod context;
pub mod domain;
pub mod error;
pub mod rgb;

use std::str::FromStr;

use crate::context::STATE;
use crate::domain::{
    request::{TransferRequest, UpdateKeyRequest},
    response::NetworkResponse,
    Metadata,
};
use crate::error::WalletError;

use base::tx::RawTransactionInfo;
use base::utils::{create_wallet, to_ic_bitcoin_network};
use candid::{CandidType, Principal};
use domain::SelfCustodyKey;
use ic_cdk::api::management_canister::bitcoin::Satoshi;
use ic_cdk::export_candid;
use serde::Deserialize;

/// Create a wallet when init the wallet canister
#[ic_cdk::init]
async fn init(args: InitArgument) {
    ic_wasi_polyfill::init(&[0u8; 32], &[]);

    let network = to_ic_bitcoin_network(&args.network);
    let steward_canister =
        Principal::from_str(&args.steward_canister).expect("Failed to parse steward canister id");

    // TODO: FIXME when bitcoin network is standby
    // let owner = ic_caller();

    // let wallet_key = SelfCustodyKey {
    //     network,
    //     owner,
    //     steward_canister,
    // };

    // Create a wallet using ECDSA Key canister and interface
    // let wallet = create_wallet(owner, steward_canister, network, key_name.clone())
    //     .await
    //     .map(|w| w.into()).expect("Failed to create first wallet in init wallet canister");

    STATE.with(|m| {
        let mut state = m.borrow_mut();

        state
            .metadata
            .set(Metadata {
                network,
                steward_canister,
                key_name: args.key_name,
                ..Default::default()
            })
            .expect("Failed to init network");

        state.controllers.insert(ic_caller(), ic_time());

        // state.raw_wallet.insert(wallet_key, wallet);
    });
}

#[ic_cdk::update]
fn issue_rgb20() -> String {
    rgb::issue_rgb20()
}

pub fn ic_caller() -> Principal {
    ic_cdk::caller()
}

pub fn ic_time() -> u64 {
    ic_cdk::api::time()
}

export_candid!();

#[derive(CandidType, Deserialize)]
struct InitArgument {
    network: String,
    steward_canister: String,
    key_name: String,
}
