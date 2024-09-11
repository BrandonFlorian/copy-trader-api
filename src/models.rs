use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackedWallet {
    pub id: Option<Uuid>,
    pub user_id: String,
    pub wallet_address: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyTradeSettings {
    pub id: Option<Uuid>,
    pub user_id: String,
    pub tracked_wallet_id: Uuid,
    pub is_enabled: bool,
    pub trade_amount_sol: f64,
    pub max_slippage: f64,
    pub max_open_positions: i32,
    pub allowed_tokens: Option<Vec<String>>,
    pub use_allowed_tokens_list: bool,
    pub allow_additional_buys: bool,
    pub match_sell_percentage: bool,
    pub min_sol_balance: f64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: String,
    pub tracked_wallet_id: Option<Uuid>,  // Make this field optional
    pub signature: String,
    pub transaction_type: String,
    pub token_address: String,
    pub amount: f64,
    pub price_sol: f64,
    pub timestamp: DateTime<Utc>,
}