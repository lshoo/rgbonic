use bitcoin::{Address, ScriptBuf};


#[derive(Clone, Debug)]
pub struct Wallet {
    // The witness script of the 2-of-2 multisig wallet.
    pub witness_script: ScriptBuf,
    // The wallet address.
    pub address: Address,
    // The derivation path of the wallet, derived from the user's principal.
    pub derivation_path: Vec<Vec<u8>>,
}