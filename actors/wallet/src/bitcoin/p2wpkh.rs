use base::{
    domain::{Wallet, WalletType},
    error::Error,
    utils::to_bitcoin_network,
};
use bitcoin::{CompressedPublicKey, PublicKey, ScriptBuf};
use candid::Principal;
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;

use crate::error::WalletError;

use super::{get_wallet_key_network_steward, insert_wallet};

// pub async fn get_or_create_p2wpkh_wallet_address(caller: Principal) -> Result<Wallet, WalletError> {
//     let (raw_wallet, wallet_key, network, steward_canister, key_name) =
//         get_wallet_key_network_steward(caller);

//     let derivation_path = principal_to_derivation_path(caller);
//     let pk_bytes =
//     match raw_wallet {
//         Some(wallet) => Ok(Wallet::from(wallet)),
//         None => {
//             let wallet = create_p2wpkh_wallet(caller, steward_canister, network, key_name).await?;

//             insert_wallet(wallet_key, wallet.clone().into());

//             Ok(wallet)
//         }
//     }
// }

/// Returns the P2WPKH address of this canister at the given derivation path.
pub async fn create_p2wpkh_wallet(
    derivation_path: Vec<Vec<u8>>,
    public_key: &[u8],
    network: BitcoinNetwork,
) -> Result<Wallet, WalletError> {
    let public_key =
        PublicKey::from_slice(public_key).map_err(|e| Error::Secp256k1Error(e.to_string()))?;

    let compressed_pk = CompressedPublicKey(public_key.inner);

    let address = bitcoin::Address::p2wpkh(&compressed_pk, to_bitcoin_network(network));

    let witness_script = ScriptBuf::p2wpkh_script_code(compressed_pk.wpubkey_hash());

    Ok(Wallet {
        witness_script,
        address,
        derivation_path,
        wallet_type: WalletType::Single,
    })
}
