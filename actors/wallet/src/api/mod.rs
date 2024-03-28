#[ic_cdk::update]
pub async fn get_wallet_address() -> String {
    crate::bitcoin::get_wallet_address().await
}
