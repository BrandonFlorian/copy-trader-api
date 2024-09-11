use axum::{
    routing::{get, post, put, delete},
    Router,
};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::db::SupabaseClient;
use crate::error::AppError;

mod routes;
mod models;
mod db;
mod error;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    let supabase_url = env::var("SUPABASE_URL")
        .map_err(|_| AppError::ConfigError("SUPABASE_URL must be set".to_string()))?;
    let supabase_service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY")
        .map_err(|_| AppError::ConfigError("SUPABASE_SERVICE_ROLE_KEY must be set".to_string()))?;
    let supabase_key = env::var("SUPABASE_API_KEY")
        .map_err(|_| AppError::ConfigError("SUPABASE_API_KEY must be set".to_string()))?;
    let user_id = env::var("USER_ID")
        .map_err(|_| AppError::ConfigError("USER_ID must be set".to_string()))?;

    let client = SupabaseClient::new(&supabase_url, &supabase_key, &supabase_service_role_key, &user_id);

    let app = Router::new()
        .route("/tracked_wallets", get(routes::get_tracked_wallets))
        .route("/tracked_wallets", post(routes::add_tracked_wallet))
        .route("/tracked_wallets/archive/:wallet_address", put(routes::archive_tracked_wallet))
        .route("/tracked_wallets/unarchive/:wallet_address", put(routes::unarchive_tracked_wallet))
        .route("/tracked_wallets/:wallet_address", delete(routes::delete_tracked_wallet))
        .route("/tracked_wallets/update", put(routes::update_tracked_wallet))
        .route("/copy_trade_settings", get(routes::get_copy_trade_settings))
        .route("/copy_trade_settings", post(routes::create_copy_trade_settings))
        .route("/copy_trade_settings", put(routes::update_copy_trade_settings))
        .route("/copy_trade_settings/:tracked_wallet_id", delete(routes::delete_copy_trade_settings))
        .route("/transaction_history", get(routes::get_transaction_history))
        .with_state(client.clone());

    let port = env::var("APP_PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse()?));

    println!("Server running on {}", addr);
    let listener = TcpListener::bind(addr).await
        .map_err(|e| AppError::ServerError(format!("Failed to bind to address: {}", e)))?;
    
    axum::serve(listener, app).await
        .map_err(|e| AppError::ServerError(format!("Server error: {}", e)))?;

    Ok(())
}