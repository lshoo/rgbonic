use crate::ic_caller;

use crate::context::RAW_WALLET;

pub async fn get_wallet_address() -> String {
    let principal = ic_caller();

    // let mut wallet = RAW_WALLET.with(|w| w.borrow_mut.);

    // let address = base::get_or_create_wallet(&mut wallet, principal).await;

    // WALLET.with(|w| w.replace(wallet););

    // address.to_string()
    todo!()
}
