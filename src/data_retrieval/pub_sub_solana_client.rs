use solana_account_decoder::UiAccountEncoding;
use solana_pubsub_client::pubsub_client::{AccountSubscription, LogsSubscription, PubsubClient};
use solana_rpc_client_api::config::{
    RpcAccountInfoConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{error::Error, str::FromStr};
pub struct PubSubSolanaClient {
    ws_url: String,
}

impl PubSubSolanaClient {
    pub fn new(ws_url: String) -> Self {
        PubSubSolanaClient { ws_url }
    }

    fn default_commitment() -> CommitmentConfig {
        CommitmentConfig::confirmed()
    }

    pub fn subscribe_account(
        &self, pubkey_str: &str,
    ) -> Result<AccountSubscription, Box<dyn Error>> {
        let pubkey = Pubkey::from_str(pubkey_str)?;

        let config = RpcAccountInfoConfig {
            encoding:         Some(UiAccountEncoding::JsonParsed),
            data_slice:       None,
            commitment:       Some(Self::default_commitment()),
            min_context_slot: None,
        };

        PubsubClient::account_subscribe(&self.ws_url, &pubkey, Some(config)).map_err(Into::into)
    }

    pub fn subscribe_logs(&self) -> Result<LogsSubscription, Box<dyn Error>> {
        let config = RpcTransactionLogsConfig {
            commitment: Some(Self::default_commitment()),
        };

        PubsubClient::logs_subscribe(&self.ws_url, RpcTransactionLogsFilter::All, config)
            .map_err(Into::into)
    }
}
