pub mod types;

use std::sync::Arc;

use ethers::{
    prelude::{k256::ecdsa::SigningKey, signer::SignerMiddlewareError},
    providers::{JsonRpcClient, Middleware, PendingTransaction},
    signers::Wallet,
};
use tracing::{error, info};

use crate::types::{TxErrors, TxStatus};

/// Wrapper for ethers client/SignerMiddleware
pub struct ClientWrapper<M> {
    pub client: Arc<M>,
}

impl<M: Middleware + 'static + JsonRpcClient> ClientWrapper<M> {
    pub fn new(client: Arc<M>) -> Self {
        Self { client }
    }
    /// Handles the transaction and returns a TxStatus
    pub async fn handle_tx(
        &self,
        tx: Result<PendingTransaction<'_, M>, SignerMiddlewareError<M, Wallet<SigningKey>>>,
    ) -> TxStatus {
        let tx = tx.map_err(|e| format!("Failed to send transaction: {:?}", e));

        let tx = match tx {
            Ok(tx) => tx,
            Err(e) => {
                error!("Failed to send transaction: {:?}", e);
                return TxStatus::Failed(TxErrors::Failed(e));
            }
        };

        let hash = tx.tx_hash();

        info!("Mining tx hash: {:?}", hash);
        let receipt = tx
            .await
            .map_err(|e| format!("Failed to mine transaction: {:?}", e));
        let receipt = match receipt {
            Ok(receipt) => receipt,
            Err(e) => return TxStatus::Failed(TxErrors::Failed(e)),
        };
        match receipt {
            Some(receipt) => {
                info!("Tx mined!");
                if receipt.status == Some(0.into()) {
                    info!("Tx reverted!");
                    return types::TxStatus::Failed(TxErrors::Reverted(receipt));
                }
                TxStatus::Successful(receipt)
            }
            None => {
                error!("No receipt for tx hash: {:?}", hash);
                // I don't even think this is possible
                TxStatus::Failed(TxErrors::NoReceipt(hash))
            }
        }
    }
}
