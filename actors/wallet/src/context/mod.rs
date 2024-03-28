use std::cell::RefCell;

use crate::domain::{RawWallet, SelfCustodyKey};

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, DefaultMemoryImpl, RestrictedMemory,
};

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
// pub type StableCell = Cell<RawSelfCustody, Memory>;

const SELF_CUSTODY_ID: MemoryId = MemoryId::new(1);

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static RAW_WALLET: RefCell<RawWalletStable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(SELF_CUSTODY_ID))
        )
    );
}

// thread_local! {
//     // The bitcoin network to connect to.
//     //
//     // When developing locally this should be `Regtest`.
//     // When deploying to the IC this should be `Testnet` or `Mainnet`.
//     // pub static NETWORK: Cell<BitcoinNetwork> = Cell::new(BitcoinNetwork::Testnet);

//     // // The ECDSA key name.
//     // pub static KEY_NAME: RefCell<String> = RefCell::new(String::from(""));

//     // // The fiduciary canister.
//     // pub static FIDUCIARY_ID: RefCell<Option<candid::Principal>> = RefCell::new(None);

//     // The custody wallet.
//     pub static WALLET: RefCell<common::CustodyData> = RefCell::default();
// }
