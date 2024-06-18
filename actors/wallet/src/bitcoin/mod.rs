use base::ICBitcoinNetwork;
use candid::Principal;

use crate::{
    context::STATE,
    domain::{RawWallet, SelfCustodyKey},
};

pub mod p2wpkh;
pub mod p2wsh;

pub use p2wpkh::*;
pub use p2wsh::*;

pub fn get_wallet_key_network_steward(
    caller: Principal,
) -> (
    Option<RawWallet>,
    SelfCustodyKey,
    ICBitcoinNetwork,
    Principal,
    String,
) {
    STATE.with(|s| {
        let state = s.borrow();
        let metadata = state.metadata.get();

        let network = metadata.network;
        let steward_canister = metadata.steward_canister;
        let key_name = metadata.key_name.clone();
        let wallet_key = SelfCustodyKey {
            network,
            owner: caller,
            steward_canister,
        };
        let wallet = state.raw_wallet.get(&wallet_key);
        (wallet, wallet_key, network, steward_canister, key_name)
    })
}

pub fn insert_wallet(wallet_key: SelfCustodyKey, wallet: RawWallet) -> Option<RawWallet> {
    STATE.with(|s| s.borrow_mut().raw_wallet.insert(wallet_key, wallet))
}
