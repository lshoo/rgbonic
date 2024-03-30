use base::ICBitcoinNetwork;
use candid::Principal;

use crate::{
    context::STATE,
    domain::{RawWallet, SelfCustodyKey, Wallet},
    error::WalletError,
};

/// Get an exist address, or generate a new address by caller
/// TODO: support multiple addresses
pub async fn get_wallet_address(caller: Principal) -> String {
    let (wallet_key, network, steward_canister) = get_wallet_key_network_steward(caller);

    let raw_wallet = get_raw_wallet(&wallet_key);

    match raw_wallet {
        Some(wallet) => Wallet::from(wallet).address.to_string(),
        None => {
            let wallet = base::utils::create_wallet(caller, steward_canister, network)
                .await
                .map_err(|e| WalletError::CreateWalletError(e.to_string()))
                .unwrap();

            STATE.with(|s| {
                s.borrow_mut()
                    .raw_wallet
                    .insert(wallet_key, wallet.clone().into())
            });

            wallet.address.to_string()
        }
    }
}

fn get_wallet_key_network_steward(
    caller: Principal,
) -> (SelfCustodyKey, ICBitcoinNetwork, Principal) {
    STATE.with(|s| {
        let state = &mut s.borrow_mut();
        let metadata = state.metadata.get();

        let network = metadata.network;
        let steward_canister = metadata.steward_canister;
        let wallet_key = SelfCustodyKey {
            network,
            owner: caller,
            steward_canister,
        };

        (wallet_key, network, steward_canister)
    })
}

fn get_raw_wallet(wallet_key: &SelfCustodyKey) -> Option<RawWallet> {
    STATE.with(|s| s.borrow_mut().raw_wallet.get(wallet_key))
}
