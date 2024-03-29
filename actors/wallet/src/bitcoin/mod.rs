use candid::Principal;

use crate::context::STATE;

pub async fn get_wallet_address(caller: &Principal) -> String {
    STATE.with(|s| {
        let state = s.borrow_mut();
        let metadata = state.metadata.get();
        let raw_wallet = &state.raw_wallet;
        let network = metadata.network;

        // let wallet = RAW_WALLET.with(|w| w.borrow_mut());

        todo!()
    })
    // let metadata = METADATA.with(|m| m.borrow().get().clone());
    // let network = metadata.network;

    // Get wallet by principal

    // let address = base::get_or_create_wallet(&mut wallet, principal).await;

    // WALLET.with(|w| w.replace(wallet););

    // address.to_string()
    // todo!()
}
