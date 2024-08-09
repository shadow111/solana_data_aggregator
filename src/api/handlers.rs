use crate::{
    api::error::ApiError,
    data_processing::{
        processor::{AccountData, BlockData, TransactionData},
        Processor,
    },
    data_retrieval::RpcSolanaClient,
};
use axum::{extract::Path, Extension, Json};
use std::sync::Arc;
use tracing::error;

pub async fn get_transaction_by_signature(
    Extension(rpc_solana_client): Extension<Arc<RpcSolanaClient>>,
    Extension(processor): Extension<Arc<Processor>>, Path(signature): Path<String>,
) -> Result<Json<TransactionData>, ApiError> {
    match rpc_solana_client.get_transaction(&signature).await {
        Ok(encoded_transaction) => match processor.process_transaction(encoded_transaction) {
            Some(transaction_data) => Ok(Json(transaction_data)),
            None => {
                error!("Transaction not found for signature {}", &signature);
                Err(ApiError::NotFound)
            }
        },
        Err(e) => {
            error!(
                "Error fetching transaction by signature {}: {:?}",
                &signature, e
            );
            Err(ApiError::InternalError)
        }
    }
}

pub async fn get_transaction_by_slot(
    Extension(rpc_solana_client): Extension<Arc<RpcSolanaClient>>,
    Extension(processor): Extension<Arc<Processor>>, Path(slot): Path<u64>,
) -> Result<Json<BlockData>, ApiError> {
    match rpc_solana_client.get_transaction_by_slot(slot).await {
        Ok(encoded_block) => match processor.process_block(encoded_block) {
            Some(block_data) => Ok(Json(block_data)),
            None => {
                error!("block not found for slot {}", &slot);
                Err(ApiError::NotFound)
            }
        },
        Err(e) => {
            error!("Error fetching block by slot {}: {:?}", &slot, e);
            Err(ApiError::InternalError)
        }
    }
}

pub async fn get_account_by_pubkey(
    Path(pubkey): Path<String>, Extension(rpc_solana_client): Extension<Arc<RpcSolanaClient>>,
    Extension(processor): Extension<Arc<Processor>>,
) -> Result<Json<AccountData>, ApiError> {
    match rpc_solana_client.get_account(&pubkey).await {
        Ok(account) => match processor.process_account(account) {
            Some(account_data) => Ok(Json(account_data)),
            None => {
                error!("Account not found {}", &pubkey);
                Err(ApiError::NotFound)
            }
        },
        Err(e) => {
            error!("Error fetching Account {}: {:?}", &pubkey, e);
            Err(ApiError::InternalError)
        }
    }
}
