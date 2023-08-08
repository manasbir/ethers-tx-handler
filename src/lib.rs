pub mod types;

use std::sync::Arc;

use ethers::{providers::{Middleware, PendingTransaction, JsonRpcClient}, prelude::{signer::SignerMiddlewareError, k256::ecdsa::SigningKey}, signers::Wallet};

use crate::types::{TxErrors, TxStatus};

/// Wrapper for ethers client/SignerMiddleware
pub struct ClientWrapper<M> {
    pub client: Arc<M>,
}

impl <M: Middleware + 'static + JsonRpcClient> ClientWrapper<M> {
    /// Handles the transaction and returns a TxStatus
    pub async fn handle_tx(&self, tx: Result<PendingTransaction<'_, M>, SignerMiddlewareError<M, Wallet<SigningKey>>,>) -> TxStatus {
        let tx = tx.map_err(|e| format!("Failed to send transaction: {:?}", e));

        let tx = match tx {
            Ok(tx) => tx,
            Err(e) => return TxStatus::Failed(TxErrors::Failed(e)),
        };
    
        let hash = tx.tx_hash();
    
        println!("Sent tx hash: {:?}", hash);
        println!("Mining...");
        let receipt = tx
            .await
            .map_err(|e| format!("Failed to mine transaction: {:?}", e));
        let receipt = match receipt {
            Ok(receipt) => receipt,
            Err(e) => return TxStatus::Failed(TxErrors::Failed(e)),
        };
        match receipt {
            Some(receipt) => {
                println!("Tx mined!");
                if receipt.status == Some(0.into()) {
                    return types::TxStatus::Failed(TxErrors::Reverted(receipt));
                }
                TxStatus::Successful(receipt)
            }
            None => {
                // I don't even think this is possible
                TxStatus::Failed(TxErrors::NoReceipt(hash))
            }
        }
    }
}