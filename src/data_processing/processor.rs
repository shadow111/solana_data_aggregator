use serde::{Deserialize, Serialize};
use solana_sdk::{
    account::Account as SolanaAccount,
    clock::{Slot, UnixTimestamp},
};
use solana_transaction_status::{
    EncodedConfirmedBlock, EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
    UiInstruction::Parsed, UiMessage, UiParsedInstruction::PartiallyDecoded, UiParsedMessage,
    UiRawMessage,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionData {
    signatures:   Vec<String>,
    blockhash:    String,
    accounts:     Vec<Account>,
    instructions: Vec<InstructionData>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pubkey:   String,
    writable: bool,
    signer:   bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InstructionData {
    program_id: String,
    data:       String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountData {
    pub lamports:   u64,
    pub data:       Vec<u8>,
    pub owner:      String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockData {
    pub previous_blockhash: String,
    pub blockhash:          String,
    pub parent_slot:        Slot,
    pub transactions:       Vec<Option<TransactionData>>,
    pub num_partitions:     Option<u64>,
    pub block_time:         Option<UnixTimestamp>,
    pub block_height:       Option<u64>,
}

pub struct Processor;

impl Processor {
    fn process_message(&self, message: UiMessage) -> Option<TransactionData> {
        match message {
            UiMessage::Parsed(parsed_message) => self.process_parsed_message(&parsed_message),
            UiMessage::Raw(raw_message) => {
                self.process_raw_message(&raw_message);
                None
            }
        }
    }

    fn process_parsed_message(&self, parsed_message: &UiParsedMessage) -> Option<TransactionData> {
        let blockhash = parsed_message.recent_blockhash.clone();
        let accounts = parsed_message
            .account_keys
            .iter()
            .map(|account| Account {
                pubkey:   account.pubkey.clone(),
                writable: account.writable,
                signer:   account.signer,
            })
            .collect();

        let instructions = parsed_message
            .instructions
            .iter()
            .filter_map(|instruction| {
                if let Parsed(PartiallyDecoded(instruction_detail)) = instruction {
                    Some(InstructionData {
                        program_id: instruction_detail.program_id.clone(),
                        data:       instruction_detail.data.clone(),
                    })
                } else {
                    println!("Unhandled instruction type");
                    None
                }
            })
            .collect();

        Some(TransactionData {
            signatures: vec![],
            blockhash,
            accounts,
            instructions,
        })
    }

    fn process_raw_message(&self, raw_message: &UiRawMessage) {
        println!("Raw message data: {:?}", raw_message);
    }

    pub fn process_transaction(
        &self, vtx: EncodedConfirmedTransactionWithStatusMeta,
    ) -> Option<TransactionData> {
        let encoded_transaction = vtx.transaction.transaction;

        match encoded_transaction {
            EncodedTransaction::Json(ui_transaction) => {
                let transaction_data = self.process_message(ui_transaction.message);
                if let Some(mut data) = transaction_data {
                    data.signatures = ui_transaction.signatures.clone();
                    return Some(data);
                }
                None
            }
            _ => {
                println!("Unsupported transaction format.");
                None
            }
        }
    }

    pub fn process_account(&self, account: SolanaAccount) -> Option<AccountData> {
        Some(AccountData {
            lamports:   account.lamports,
            data:       account.data,
            owner:      account.owner.to_string(),
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        })
    }

    fn process_encoded_transaction(&self, vtx: EncodedTransaction) -> Option<TransactionData> {
        match vtx {
            EncodedTransaction::Json(ui_transaction) => {
                let transaction_data = self.process_message(ui_transaction.message);
                if let Some(mut data) = transaction_data {
                    data.signatures = ui_transaction.signatures.clone();
                    return Some(data);
                }
                None
            }
            _ => {
                println!("Unsupported transaction format.");
                None
            }
        }
    }
    pub fn process_block(&self, block: EncodedConfirmedBlock) -> Option<BlockData> {
        Some(BlockData {
            previous_blockhash: block.previous_blockhash,
            blockhash:          block.blockhash,
            parent_slot:        block.parent_slot,
            transactions:       block
                .transactions
                .iter()
                .map(|tx| self.process_encoded_transaction(tx.transaction.clone()))
                .collect(),
            num_partitions:     block.num_partitions,
            block_time:         block.block_time,
            block_height:       block.block_height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use solana_transaction_status::{
        EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction,
        EncodedTransactionWithStatusMeta, UiMessage,
        UiParsedMessage, UiTransaction,
    };

    #[test]
    fn test_process_transaction_json() {
        let processor = Processor;

        let ui_transaction = UiTransaction {
            signatures: vec!["Signature1".to_string()],
            message:    UiMessage::Parsed(UiParsedMessage {
                account_keys:          vec![],
                instructions:          vec![],
                recent_blockhash:      "Blockhash".to_string(),
                address_table_lookups: None,
            }),
        };

        let encoded_transaction = EncodedConfirmedTransactionWithStatusMeta {
            slot:        0,
            transaction: EncodedTransactionWithStatusMeta {
                transaction: EncodedTransaction::Json(ui_transaction),
                meta:        None,
                version:     None,
            },
            block_time:  None,
        };

        let result = processor.process_transaction(encoded_transaction);

        assert!(result.is_some());
        let transaction_data = result.unwrap();
        assert_eq!(transaction_data.signatures.len(), 1);
        assert_eq!(transaction_data.blockhash, "Blockhash");
    }

    #[test]
    fn test_process_account() {
        let processor = Processor;

        let pubkey = Pubkey::new_unique();
        let solana_account = SolanaAccount {
            lamports:   1000,
            data:       vec![1, 2, 3],
            owner:      pubkey,
            executable: false,
            rent_epoch: 0,
        };

        let result = processor.process_account(solana_account);

        assert!(result.is_some());
        let account_data = result.unwrap();
        assert_eq!(account_data.lamports, 1000);
        assert_eq!(account_data.data, vec![1, 2, 3]);
        assert_eq!(account_data.owner, pubkey.to_string());
        assert!(!account_data.executable);
    }

    #[test]
    fn test_process_block() {
        let processor = Processor;

        let block = EncodedConfirmedBlock {
            previous_blockhash: "PreviousBlockhash".to_string(),
            blockhash:          "Blockhash".to_string(),
            parent_slot:        0,
            transactions:       vec![],
            rewards:            vec![],
            block_time:         None,
            block_height:       None,
            num_partitions:     None,
        };

        let result = processor.process_block(block);

        assert!(result.is_some());
        let block_data = result.unwrap();
        assert_eq!(block_data.previous_blockhash, "PreviousBlockhash");
        assert_eq!(block_data.blockhash, "Blockhash");
        assert_eq!(block_data.parent_slot, 0);
        assert!(block_data.transactions.is_empty());
    }

    #[test]
    fn test_process_encoded_transaction_unsupported_format() {
        let processor = Processor;

        let encoded_transaction = EncodedTransaction::LegacyBinary("tx".to_string());

        let result = processor.process_encoded_transaction(encoded_transaction);

        assert!(result.is_none());
    }
}
