use base::ICBitcoinNetwork;
use bitcoin::Address;
use candid::Principal;

use crate::{domain::Wallet, error::WalletError};

use super::{get_wallet_key_network_steward, insert_wallet};

/// Get an exist address, or generate a new address by caller
/// Returns a address if success, or returns `CreateWalletFailed`
/// TODO: support multiple addresses
pub async fn get_or_create_p2wsh_address_str(caller: Principal) -> Result<String, WalletError> {
    get_or_create_p2wsh_address(caller)
        .await
        .map(|a| a.to_string())
}

pub async fn get_or_create_p2wsh_address(caller: Principal) -> Result<Address, WalletError> {
    get_or_create_p2wsh_wallet(caller).await.map(|w| w.address)
}

pub async fn get_or_create_p2wsh_wallet(caller: Principal) -> Result<Wallet, WalletError> {
    let (raw_wallet, wallet_key, network, steward_canister, key_name) =
        get_wallet_key_network_steward(caller);

    match raw_wallet {
        Some(wallet) => Ok(Wallet::from(wallet)),
        None => {
            let wallet = create_wallet(caller, steward_canister, network, key_name).await?;

            insert_wallet(wallet_key, wallet.clone().into());

            Ok(wallet)
        }
    }
}

async fn create_wallet(
    caller: Principal,
    steward_canister: Principal,
    network: ICBitcoinNetwork,
    key_name: String,
) -> Result<Wallet, WalletError> {
    base::utils::create_p2wsh_multisig22_wallet(caller, steward_canister, network, key_name)
        .await
        .map_err(|e| WalletError::CreateWalletError(e.to_string()))
}
