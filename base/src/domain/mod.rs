use bitcoin::Address;

pub struct UserWallet {
    pub address: Address,
    pub derivation_path: Vec<Vec<u8>>,
}
