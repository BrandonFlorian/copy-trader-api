use postgrest::Postgrest;
use serde_json::json;
use uuid::Uuid;
use reqwest;
use std::error::Error;

use crate::models::{TrackedWallet, CopyTradeSettings, Transaction};
use crate::error::AppError;

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::RequestError(error.to_string())
    }
}

impl From<Box<dyn Error>> for AppError {
    fn from(error: Box<dyn Error>) -> Self {
        AppError::PostgrestError(error.to_string())
    }
}

#[derive(Clone)]
pub struct SupabaseClient {
    client: Postgrest,
    user_id: String,
}

impl SupabaseClient {
    pub fn new(url: &str, api_key: &str, user_id: &str) -> Self {
        let client = Postgrest::new(url)
            .insert_header("apikey", api_key)
            .insert_header("Authorization", format!("Bearer {}", api_key));
        
        Self {
            client,
            user_id: user_id.to_string(),
        }
    }

    pub async fn get_tracked_wallets(&self) -> Result<Vec<TrackedWallet>, AppError> {
        let resp = self.client
            .from("tracked_wallets")
            .select("*")
            .eq("user_id", &self.user_id)
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;

        serde_json::from_str(&body)
            .map_err(|e| AppError::JsonParseError(e.to_string()))
    }

    pub async fn add_tracked_wallet(&self, wallet: TrackedWallet) -> Result<Uuid, AppError> {
        let insert_data = serde_json::json!({
            "user_id": self.user_id,
            "wallet_address": wallet.wallet_address,
            "is_active": wallet.is_active
        });

        let resp = self.client
            .from("tracked_wallets")
            .insert(insert_data.to_string())
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;

        let inserted: Vec<TrackedWallet> = serde_json::from_str(&body)
            .map_err(|e| AppError::JsonParseError(e.to_string()))?;
        Ok(inserted[0].id.unwrap())
    }

    pub async fn archive_tracked_wallet(&self, wallet_address: &str) -> Result<String, AppError> {
        let resp = self.client
            .from("tracked_wallets")
            .update(json!({"is_active": false}).to_string())
            .eq("user_id", &self.user_id)
            .eq("wallet_address", wallet_address)
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;
        let updated: Vec<TrackedWallet> = serde_json::from_str(&body)?;
        Ok(format!("Archived wallet: {}", updated[0].wallet_address))
    }

    pub async fn unarchive_tracked_wallet(&self, wallet_address: &str) -> Result<String, AppError> {
        let resp = self.client
            .from("tracked_wallets")
            .update(json!({"is_active": true}).to_string())
            .eq("user_id", &self.user_id)
            .eq("wallet_address", wallet_address)
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;
        let updated: Vec<TrackedWallet> = serde_json::from_str(&body)?;
        Ok(format!("Unarchived wallet: {}", updated[0].wallet_address))
    }

    pub async fn delete_tracked_wallet(&self, wallet_address: &str) -> Result<String, AppError> {
        let resp = self.client
            .from("tracked_wallets")
            .delete()
            .eq("user_id", &self.user_id)
            .eq("wallet_address", wallet_address)
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;
        if resp.status() == 204 {
            Ok("Tracked wallet deleted successfully".to_string())
        } else {
            Err(AppError::DatabaseError("Failed to delete tracked wallet".to_string()))
        }
    }

    pub async fn update_tracked_wallet(&self, wallet: TrackedWallet) -> Result<Uuid, AppError> {
        let resp = self.client
            .from("tracked_wallets")
            .update(json!({
                "wallet_address": wallet.wallet_address,
                "is_active": wallet.is_active
            }).to_string())
            .eq("user_id", &self.user_id)
            .eq("id", wallet.id.unwrap().to_string())
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;
        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;
        let updated: Vec<TrackedWallet> = serde_json::from_str(&body)?;
        Ok(updated[0].id.unwrap())
    }

    pub async fn get_copy_trade_settings(&self) -> Result<Vec<CopyTradeSettings>, AppError> {
        let resp = self.client
            .from("copy_trade_settings")
            .select("*")
            .eq("user_id", &self.user_id)
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;
        let settings: Vec<CopyTradeSettings> = serde_json::from_str(&body)?;
        Ok(settings)
    }

    pub async fn create_copy_trade_settings(&self, settings: CopyTradeSettings) -> Result<Uuid, AppError> {
        let resp = self.client
            .from("copy_trade_settings")
            .insert(json!({
                "user_id": self.user_id,
                "tracked_wallet_id": settings.tracked_wallet_id,
                "is_enabled": settings.is_enabled,
                "trade_amount_sol": settings.trade_amount_sol,
                "max_slippage": settings.max_slippage,
                "max_open_positions": settings.max_open_positions,
                "allowed_tokens": settings.allowed_tokens,
                "use_allowed_tokens_list": settings.use_allowed_tokens_list,
                "allow_additional_buys": settings.allow_additional_buys,
                "match_sell_percentage": settings.match_sell_percentage,
                "min_sol_balance": settings.min_sol_balance
            }).to_string())
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;
        let inserted: Vec<CopyTradeSettings> = serde_json::from_str(&body)?;
        Ok(inserted[0].id.unwrap())
    }

    pub async fn update_copy_trade_settings(&self, settings: CopyTradeSettings) -> Result<Uuid, AppError> {
        let resp = self.client
            .from("copy_trade_settings")
            .update(json!({
                "is_enabled": settings.is_enabled,
                "trade_amount_sol": settings.trade_amount_sol,
                "max_slippage": settings.max_slippage,
                "max_open_positions": settings.max_open_positions,
                "allowed_tokens": settings.allowed_tokens,
                "use_allowed_tokens_list": settings.use_allowed_tokens_list,
                "allow_additional_buys": settings.allow_additional_buys,
                "match_sell_percentage": settings.match_sell_percentage,
                "min_sol_balance": settings.min_sol_balance
            }).to_string()) // Convert JSON to String
            .eq("user_id", &self.user_id)
            .eq("tracked_wallet_id", settings.tracked_wallet_id.to_string())
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;
        let updated: Vec<CopyTradeSettings> = serde_json::from_str(&body)?;
        Ok(updated[0].id.unwrap())
    }

    pub async fn delete_copy_trade_settings(&self, tracked_wallet_id: Uuid) -> Result<String, AppError> {
        let resp = self.client
            .from("copy_trade_settings")
            .delete()
            .eq("user_id", &self.user_id)
            .eq("tracked_wallet_id", tracked_wallet_id.to_string())
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        if resp.status() == 204 {
            Ok("Copy trade settings deleted successfully".to_string())
        } else {
            Err(AppError::DatabaseError("Failed to delete copy trade settings".to_string()))
        }
    }

    pub async fn get_transaction_history(&self) -> Result<Vec<Transaction>, AppError> {
        let resp = self.client
            .from("transactions")
            .select("*")
            .eq("user_id", &self.user_id)
            .execute()
            .await
            .map_err(|e| AppError::PostgrestError(e.to_string()))?;

        let body = resp.text().await
            .map_err(|e| AppError::RequestError(e.to_string()))?;
        let transactions: Vec<Transaction> = serde_json::from_str(&body)?;
        Ok(transactions)
    }

    // pub async fn log_transaction(&self, transaction: Transaction) -> Result<Uuid, AppError> {
    //     let resp = self.client
    //         .from("transactions")
    //         .insert(json!({
    //             "user_id": self.user_id,
    //             "tracked_wallet_id": transaction.tracked_wallet_id,
                
    //         }).to_string()) // Convert JSON to String
    //         .execute()
    //         .await
    //         .map_err(|e| AppError::PostgrestError(e.to_string()))?;

    //     let body = resp.text().await
    //         .map_err(|e| AppError::RequestError(e.to_string()))?;
    //     let inserted: Vec<Transaction> = serde_json::from_str(&body)?;
    //     Ok(inserted[0].id.unwrap())
    // }
}