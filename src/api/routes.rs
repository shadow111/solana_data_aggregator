use axum::{routing::get, Extension, Router};
use std::sync::Arc;

use crate::{
    api::handlers::{get_account_by_pubkey, get_transaction_by_signature, get_transaction_by_slot},
    data_processing::Processor,
    data_retrieval::RpcSolanaClient,
};

pub fn create_router(rpc_solana_client: Arc<RpcSolanaClient>, processor: Arc<Processor>) -> Router {
    Router::new()
        .route(
            "/api/transaction/signature/:signature",
            get(get_transaction_by_signature),
        )
        .route("/api/transaction/slot/:slot", get(get_transaction_by_slot))
        .route("/api/account/:pubkey", get(get_account_by_pubkey))
        .layer(Extension(rpc_solana_client))
        .layer(Extension(processor))
}
