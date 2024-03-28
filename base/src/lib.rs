pub mod api;
pub mod constants;
pub mod domain;
pub mod error;
pub mod tx;
pub mod utils;

use bitcoin::EcdsaSighashType;

pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

pub type ICBitcoinNetwork = ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
