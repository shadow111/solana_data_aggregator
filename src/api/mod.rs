// use crate::{
// api::routes::{get_transaction_by_day, get_transaction_by_signature},
// data_processing::Processor,
// data_retrieval::RpcSolanaClient,
// };
// use axum::{routing::get, Router};
// use std::sync::Arc;
//
// pub mod error;
// pub mod routes;
//
// pub fn create_router(rpc_solana_client: &RpcSolanaClient, processor: &Processor) -> Router {
// Router::new()
// .route(
// "/transaction/signature/:signature",
// get(|params| get_transaction_by_signature(rpc_solana_client, processor, params)),
// )
// .route(
// "/transaction/day/:day",
// get(|params| get_transaction_by_day(rpc_solana_client, processor, params)),
// )
// .route("/account/:pubkey", get(get_account_by_pubkey))
// }
