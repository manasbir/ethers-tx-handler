use ethers::types::{Address, H256, U64, TransactionReceipt};
use serde::{Deserialize, Serialize};

// minimal types since they are pretty case specific

#[derive(Debug, Serialize, Clone)]
pub enum TxStatus {
    Successful(TransactionReceipt),
    Failed(TxErrors),
}

#[derive(Debug, Serialize, Clone)]
pub enum TxErrors {
    NoReceipt(H256),
    Reverted(TransactionReceipt),
    Failed(String),
}
