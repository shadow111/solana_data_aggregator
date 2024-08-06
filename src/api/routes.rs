/*use crate::{
    api::error::ApiError,
    data_processing::{processor, processor::TransactionData, Processor},
    data_retrieval::RpcSolanaClient,
};
use axum::{
    extract::{Path, Query},
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tracing::{error, info};

#[derive(Deserialize)]
pub struct TransactionIdQuery {
    id: String,
}

#[derive(Deserialize)]
pub struct TransactionDayQuery {
    day: String,
}

pub async fn get_transaction_by_signature(
    rpc_solana_client: &RpcSolanaClient, processor: &Processor,
    Query(params): Query<TransactionIdQuery>,
) -> Result<Json<TransactionData>, ApiError> {
    match rpc_solana_client.get_transaction(&params.id).await {
        Ok(encoded_transaction) => match processor.process_transaction(encoded_transaction) {
            Some(transaction_data) => Ok(Json(transaction_data)),
            None => {
                error!("Transaction not found for signature {}", &params.id);
                Err(ApiError::NotFound)
            }
        },
        Err(e) => {
            error!(
                "Error fetching transaction by signature {}: {:?}",
                &params.id, e
            );
            Err(ApiError::InternalError)
        }
    }
}

pub async fn get_transaction_by_day(
    rpc_solana_client: &RpcSolanaClient, processor: &Processor,
    Query(params): Query<TransactionIdQuery>,
) -> Result<Json<TransactionData>, ApiError> {
    match rpc_solana_client.get_transaction(&params.id).await {
        Ok(encoded_transaction) => match processor.process_transaction(encoded_transaction) {
            Some(transaction_data) => Ok(Json(transaction_data)),
            None => {
                error!("Transaction not found for signature {}", &params.id);
                Err(ApiError::NotFound)
            }
        },
        Err(e) => {
            error!(
                "Error fetching transaction by signature {}: {:?}",
                &params.id, e
            );
            Err(ApiError::InternalError)
        }
    }
}
*/
// async fn get_transaction_by_day(Query(params): Query<TransactionDayQuery>) -> Result<Json<Vec<ProcessedData>>, ApiError> {
// match processor::get_transaction_by_day(&params.day).await {
// Ok(data) => Ok(Json(data)),
// Err(e) => {
// tracing::error!("Error fetching transactions for day {}: {:?}", &params.day, e);
// Err(ApiError::InternalError)
// }
// }
// }
