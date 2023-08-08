pub mod types;

use ethers::{
    prelude::signer::SignerMiddlewareError,
    providers::{JsonRpcClient, Middleware, PendingTransaction},
    signers::Signer,
};
use tracing::{error, info};

use crate::types::{TxErrors, TxStatus};


/// Handles the transaction and returns a TxStatus
pub async fn handle_tx<P: JsonRpcClient, S: Signer>(
    tx: Result<PendingTransaction<'_, P>, SignerMiddlewareError<impl Middleware, S>>,
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


#[cfg(test)]
mod tests {
    use ethers::{
        prelude::SignerMiddleware,
        signers::{LocalWallet, Signer},
        types::TransactionRequest,
        utils::{parse_ether, Anvil},
    };

    use crate::handle_tx;

    #[tokio::test]
    async fn it_works() {
        use ethers::providers::{Http, Middleware, Provider};
        use std::sync::Arc;

        let anvil = Anvil::new().spawn();
        let signer: LocalWallet = anvil.keys()[0].clone().into();
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();

        let client = SignerMiddleware::new(provider.clone(), signer.with_chain_id(31337u64));
        let client = Arc::new(client);

        let tx = TransactionRequest::pay(anvil.addresses()[0], 100);


        let tx = client.send_transaction(tx, None).await;
        let status = handle_tx(tx).await;
        println!("{:?}", status);

        let tx_2 = TransactionRequest::pay(anvil.addresses()[1], parse_ether("10000000").unwrap());
        let tx = client.send_transaction(tx_2, None).await;

        let status = handle_tx(tx).await;
        println!("{:?}", status);
    }
}
