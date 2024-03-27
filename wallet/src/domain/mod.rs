



use bitcoin::Address;
use candid::{CandidType, Deserialize};
use ic_stable_structures::{storable::Bound, Storable};

use crate::constants::ECDSA_SIZE;


#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct ECDSAKey {
    pub key: String,
}

impl Storable for ECDSAKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        // String already implements `Storable`.
        self.key.to_bytes()
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Self {
            key: String::from_bytes(bytes)
        }
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: ECDSA_SIZE as u32,
        is_fixed_size: false,
    };
}

pub struct UserWallet {
    pub address: Address,
    pub derivation_path: Vec<Vec<u8>>,
}