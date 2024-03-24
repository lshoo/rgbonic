use bitcoin::{Address, EcdsaSighashType};

pub mod api;
pub mod error;
pub mod tx;

pub const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

pub struct UserWallet {
    pub address: Address,
    pub derivation_path: Vec<Vec<u8>>,
}