use std::cell::RefCell;

use crate::domain::{Metadata, RawWallet, SelfCustodyKey};

use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, Cell as StableCell, DefaultMemoryImpl, RestrictedMemory,
};

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

pub type RawWalletStable = StableBTreeMap<SelfCustodyKey, RawWallet, Memory>;
// pub type StableCell = Cell<RawSelfCustody, Memory>;

const METADATA_PAGES: u64 = 64;

const SELF_CUSTODY_ID: MemoryId = MemoryId::new(1);
const CONTROLLER_ID: MemoryId = MemoryId::new(2);

thread_local! {

    // pub static METADATA: RefCell<StableCell<Metadata, RM>> =
    // RefCell::new(StableCell::init(
    //     RM::new(DefMem::default(), 0..METADATA_PAGES),
    //     Metadata::default(),
    //   ).expect("failed to initialize the metadata cell")
    // );


    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // pub static RAW_WALLET: RefCell<RawWalletStable> = RefCell::new(
    //     StableBTreeMap::init(
    //         MEMORY_MANAGER.with(|m| m.borrow().get(SELF_CUSTODY_ID))
    //     )
    // );

    pub static STATE: RefCell<State> = RefCell::new(
        State {
            metadata: StableCell::init(
                RM::new(DefMem::default(), 0..METADATA_PAGES),
                Metadata::default(),
              ).expect("failed to initialize the metadata cell"),
            raw_wallet: StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(SELF_CUSTODY_ID))
            ),
            controllers: StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(CONTROLLER_ID))
            ),
        }
    )
}

pub struct State {
    pub metadata: StableCell<Metadata, RM>,
    pub raw_wallet: RawWalletStable,
    pub controllers: StableBTreeMap<Principal, u64, Memory>,
}