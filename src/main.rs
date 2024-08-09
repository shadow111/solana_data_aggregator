mod api;
mod config;
mod data_processing;
mod data_retrieval;
mod data_storage;
use crate::{api::routes, data_retrieval::PubSubSolanaClient};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use config::Config;
use data_processing::Processor;
use data_retrieval::RpcSolanaClient;
use log::warn;
use std::{env, error::Error, net::SocketAddr, sync::Arc};
use tower_http::cors::CorsLayer;
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
    let rpc_solana_client = Arc::new(RpcSolanaClient::new(&config.solana_rpc_url));
    let processor = Arc::new(Processor {});

    let pub_sub_solana_client = Arc::new(PubSubSolanaClient::new(config.solana_ws_url));

    let _recent_blockhash = rpc_solana_client.get_recent_blockhash().await?;

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

    // Spawn a task for the logs subscription
    let pub_sub_solana_client_clone = Arc::clone(&pub_sub_solana_client);
    let logs_subscription_task = tokio::spawn(async move {
        if let Ok(logs_subscription) = pub_sub_solana_client_clone.subscribe_logs() {
            let (mut _log_subscription_client, log_subscription_receiver) = logs_subscription;
            while let Ok(response) = log_subscription_receiver.recv() {
                info!("logs subscription response: {:?}", response);
            }
            error!("Error or disconnect in logs subscription");
        } else {
            error!("Error subscribing to logs");
        }
    });

    let pub_sub_solana_client_clone = Arc::clone(&pub_sub_solana_client);
    let account_subscription_task = tokio::spawn(async move {
        if let Ok(account_subscription) =
            pub_sub_solana_client_clone.subscribe_account(&config.account_pubkey)
        {
            let (mut _account_subscription_client, account_subscription_receiver) =
                account_subscription;
            while let Ok(response) = account_subscription_receiver.recv() {
                info!("account subscription response: {:?}", response);
            }
            error!("Error or disconnect in account subscription");
        } else {
            error!("Error subscribing to account updates");
        }
    });

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = routes::create_router(rpc_solana_client, processor).layer(cors);
    let addr = SocketAddr::new(config.api_bind_address.parse()?, config.port.parse()?);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // Await the spawned tasks to ensure they keep running
    tokio::try_join!(logs_subscription_task, account_subscription_task)?;

    Ok(())
}
