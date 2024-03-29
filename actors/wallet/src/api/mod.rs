use ic_cdk::update;

#[update]
pub async fn get_wallet_address() -> String {
    crate::bitcoin::get_wallet_address().await
}
