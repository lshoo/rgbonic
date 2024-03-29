use std::{collections::HashMap, str::FromStr};

use base::ICBitcoinNetwork;
use bitcoin::{Address, ScriptBuf};
use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;

use crate::constants::{METADATA_SIZE, SELF_CUSTODY_SIZE};

#[derive(Debug, Clone, CandidType, Deserialize, Default)]
pub struct Metadata {
    pub network: ICBitcoinNetwork,
    pub key: String,
    pub updated_time: u64,
}

impl Storable for Metadata {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: METADATA_SIZE as u32,
        is_fixed_size: false,
    };
}

#[derive(Clone, Debug)]
pub struct Wallet {
    // The witness script of the 2-of-2 multisig wallet.
    pub witness_script: ScriptBuf,
    // The wallet address.
    pub address: Address,
    // The derivation path of the wallet, derived from the user's principal.
    pub derivation_path: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct RawWallet {
    pub witness_scripte: Vec<u8>,
    pub address: String,
    pub derivation_path: Vec<Vec<u8>>,
}

impl From<RawWallet> for Wallet {
    fn from(wallet: RawWallet) -> Self {
        Self {
            witness_script: ScriptBuf::from_bytes(wallet.witness_scripte),
            address: Address::from_str(&wallet.address).unwrap().assume_checked(),
            derivation_path: wallet.derivation_path,
        }
    }
}

/// Included a Self Custody wallet data to sign transaction
#[derive(Clone, Debug)]
pub struct SelfCustody {
    pub network: ICBitcoinNetwork,
    pub key_name: String,
    pub steward_canister: Principal,
    pub wallets: HashMap<Principal, Wallet>,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelfCustodyKey {
    pub network: ICBitcoinNetwork,
    pub key_name: String,
    pub steward_canister: Principal,
}

// #[derive(Clone, nDebug, CandidType, Deserialize)]
// pub struct RawSelfCustody {
//     pub network: ICBitcoinNetwork,
//     pub key_name: String,
//     pub steward_canister: Principal,
//     pub wallets: HashMap<Principal, RawWallet>,
// }

// impl From<RawSelfCustody> for SelfCustody {
//     fn from(custody: RawSelfCustody) -> Self {
//         Self {
//             network: custody.network,
//             key_name: custody.key_name,
//             steward_canister: custody.steward_canister,
//             // TODO: FIX ME for large size
//             wallets: custody
//                 .wallets
//                 .into_iter()
//                 .map(|(k, v)| (k, v.into()))
//                 .collect(),
//         }
//     }
// }

// impl Storable for RawSelfCustody {
//     fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
//         std::borrow::Cow::Owned(Encode!(self).unwrap())
//     }

//     fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
//         Decode!(bytes.as_ref(), Self).unwrap()
//     }

//     const BOUND: Bound = Bound::Bounded {
//         max_size: SELF_CUSTODY_SIZE as u32,
//         is_fixed_size: false,
//     };
// }

impl Storable for SelfCustodyKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: SELF_CUSTODY_SIZE as u32,
        is_fixed_size: false,
    };
}

impl Storable for RawWallet {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: SELF_CUSTODY_SIZE as u32,
        is_fixed_size: false,
    };
}


#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UpdateKeyRequest {
    pub new_key: String,
    pub old_key: String,
}
