use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    pub input_mint: String,
    pub output_mint: String,
    pub input_amount: String,
    pub output_amount: String,
    pub timestamp: u64,
    pub rate: String,
    pub tx_hash: String,
    pub platform: String,
    pub fee_payer_public_key: String,
    pub is_gasless: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupActivityResponse {
    pub histories: Vec<Activity>,
    pub has_more_data: bool,
}
