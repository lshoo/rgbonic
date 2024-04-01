use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct UpdateKeyRequest {
    pub new_key: String,
    pub old_key: String,
}
