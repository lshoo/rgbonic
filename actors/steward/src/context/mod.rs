use std::cell::RefCell;

use crate::domain::ECDSAKey;
use candid::Principal;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    BTreeMap as StableBTreeMap, DefaultMemoryImpl, RestrictedMemory,
};

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

pub type ECDSAKeyStable = StableBTreeMap<Principal, ECDSAKey, Memory>;

const ECDSA_KEY_ID: MemoryId = MemoryId::new(1);

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static ECDSA_KEYS: RefCell<ECDSAKeyStable> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(ECDSA_KEY_ID))
        )
    );
}
