mod api;
mod config;
mod data_processing;
mod data_retrieval;
mod data_storage;
use crate::{data_retrieval::PubSubSolanaClient};
use axum::Router;
use config::Config;
use data_processing::Processor;
use data_retrieval::RpcSolanaClient;
use log::warn;
use std::{env, error::Error, net::SocketAddr, str::FromStr};
use tokio::time::sleep;
use tracing::{error, info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Load configuration
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| {
        warn!("CONFIG_PATH not set, using default 'config.toml'");
        "config.toml".to_string()
    });
    let config = Config::from_file(&config_path)?;

    // Initialize Solana client
    let rpc_solana_client = RpcSolanaClient::new(&config.solana_rpc_url);
    let pub_sub_solana_client = PubSubSolanaClient::new(config.solana_ws_url);

    let processor = Processor {};
    let recent_blockhash = rpc_solana_client.get_recent_blockhash().await?;

    // Process transaction
    match rpc_solana_client
        .get_transaction(&config.transaction_signature)
        .await
    {
        Ok(encoded_transaction) => {
            if let Some(transaction_data) = processor.process_transaction(encoded_transaction) {
                info!("Processed Transaction Data: {:?}", transaction_data);
            } else {
                info!("Failed to process the transaction or unsupported format");
            }
        }
        Err(e) => error!("Failed to fetch transaction: {}", e),
    }

    // Process account information
    match rpc_solana_client.get_account(&config.account_pubkey).await {
        Ok(account) => {
            if let Some(account_data) = processor.process_account(account) {
                info!("Processed Account Data: {:?}", account_data);
            } else {
                info!("Failed to process Account");
            }
        }
        Err(e) => error!("Failed to fetch account: {}", e),
    }

    // Subscribe to logs and handle them properly
    if let Ok(logs_subscription) = pub_sub_solana_client.subscribe_logs() {
        let (mut _log_subscription_client, mut log_subscription_receiver) = logs_subscription;
        while let Ok(response) = log_subscription_receiver.recv() {
            info!("logs subscription response: {:?}", response);
        }
        error!("Error or disconnect in logs subscription");
    } else {
        error!("Error subscribing to logs");
    }

    // Subscribe to account updates
    if let Ok(account_subscription) =
        pub_sub_solana_client.subscribe_account(&config.account_pubkey)
    {
        let (mut _account_subscription_client, mut account_subscription_receiver) =
            account_subscription;
        while let Ok(response) = account_subscription_receiver.recv() {
            info!("account subscription response: {:?}", response);
        }
        error!("Error or disconnect in account subscription");
    } else {
        error!("Error subscribing to account updates");
    }

    // // Build our application with routes
    // let app = create_router(&rpc_solana_client, &processor);
    //
    // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    // .await
    // .unwrap();
    // axum::serve(listener, app).await.unwrap();

    Ok(())
}
