pub mod api;
pub mod domain;
pub mod state;

use std::cell::RefCell;

use ic_cdk::init;

use wallet::{constants::ECDSA_SIZE, domain::ECDSAKey};

use ic_stable_structures::Cell as StableCell;
use ic_stable_structures::{
    memory_manager::{MemoryManager, VirtualMemory},
    DefaultMemoryImpl, RestrictedMemory,
};

pub type DefMem = DefaultMemoryImpl;
pub type RM = RestrictedMemory<DefMem>;
pub type VM = VirtualMemory<RM>;

pub type Memory = VirtualMemory<DefMem>;

thread_local! {

    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    // The ECDSA Key
    static ECDSA_KEY: RefCell<StableCell<ECDSAKey, RM>> = RefCell::new(
        StableCell::init(
            RM::new(DefMem::default(), 0..ECDSA_SIZE),
            ECDSAKey::default(),
        ).expect("Failed to initialize the ECDSA key")
    );
}

#[init]
pub fn init(key: ECDSAKey) {
    ECDSA_KEY.with(|k| {
        k.borrow_mut()
            .set(key)
            .expect("Failed to initialize the ECDSA key");
    });
}
