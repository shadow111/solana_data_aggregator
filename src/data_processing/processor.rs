use serde::{Deserialize, Serialize};
use solana_sdk::account::Account as SolanaAccount;
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, EncodedTransaction, UiInstruction::Parsed,
    UiMessage, UiParsedInstruction::PartiallyDecoded, UiParsedMessage, UiRawMessage,
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

#[derive(Debug)]
pub struct AccountData {
    pub lamports:   u64,
    pub data:       Vec<u8>,
    pub owner:      String,
    pub executable: bool,
    pub rent_epoch: u64,
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
}
