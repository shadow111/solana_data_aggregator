use solana_account_decoder::UiAccountEncoding;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcAccountInfoConfig};
use solana_sdk::{
    account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature,
};
use solana_transaction_status::{
    self, EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding,
};
use std::{error::Error, str::FromStr, sync::Arc};

pub struct RpcSolanaClient {
    rpc_client: Arc<RpcClient>,
}

impl RpcSolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        let rpc_client =
            RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
        RpcSolanaClient {
            rpc_client: Arc::new(rpc_client),
        }
    }

    pub async fn get_recent_blockhash(&self) -> Result<String, Box<dyn Error>> {
        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;
        Ok(recent_blockhash.to_string())
    }

    pub async fn get_transaction(
        &self, signature: &str,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, Box<dyn Error>> {
        let signature = Signature::from_str(signature)?;
        self.rpc_client
            .get_transaction(&signature, UiTransactionEncoding::JsonParsed)
            .await
            .map_err(|e| {
                eprintln!("Failed to fetch the transaction: {:?}", e);
                "Failed to decode transaction. The transaction might be corrupted or unsupported."
                    .into()
            })
    }

    pub async fn get_account(&self, pubkey_str: &str) -> Result<Account, Box<dyn Error>> {
        let pubkey = Pubkey::from_str(pubkey_str)?;

        let commitment_config = CommitmentConfig::processed();
        let config = RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::JsonParsed),
            commitment: Some(commitment_config),
            ..RpcAccountInfoConfig::default()
        };

        let response = self
            .rpc_client
            .get_account_with_config(&pubkey, config)
            .await
            .map_err(|e| {
                eprintln!("Failed to fetch the account: {:?}", e);
                Box::new(e) as Box<dyn Error>
            })?;

        match response.value {
            Some(account) => Ok(account),
            None => Err("Account not found.".into()),
        }
    }
}
