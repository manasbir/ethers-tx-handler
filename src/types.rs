use ethers::types::{TransactionReceipt, H256};
use serde::Serialize;

// minimal types since they are pretty case specific

/// enum for success and failure
#[derive(Debug, Serialize, Clone)]
pub enum TxStatus {
    /// Transaction was successful and executed
    Successful(TransactionReceipt),
    /// Transaction failed in one way or another
    Failed(TxErrors),
}

/// enum for the 3 different types of failures
#[derive(Debug, Serialize, Clone)]
pub enum TxErrors {
    /// Transaction went through, but reciept is None
    NoReceipt(H256),
    /// Transaction reverted
    Reverted(TransactionReceipt),
    /// Transaction failed to send or mine
    Failed(String),
}
