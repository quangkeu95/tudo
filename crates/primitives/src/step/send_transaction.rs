use crate::{Step, StepError, StepOutput};
use derive_builder::Builder;
use ethers::middleware::signer::SignerMiddlewareError;
use ethers::prelude::{Middleware, ProviderError, Signer, SignerMiddleware, TransactionReceipt};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::BlockId;
use thiserror::Error;

/// Allow to send transaction using a [`SignerMiddleware`]
#[derive(Debug, Builder)]
pub struct SendTransaction<M: Middleware, S: Signer, Tx>
where
    Tx: Into<TypedTransaction> + Send + Sync + Clone,
{
    pub signer_middleware: SignerMiddleware<M, S>,
    pub tx_request: Tx,
    #[builder(default)]
    pub block: Option<BlockId>,
}

#[async_trait::async_trait]
impl<M, S, Tx> Step for SendTransaction<M, S, Tx>
where
    M: Middleware,
    S: Signer,
    Tx: Into<TypedTransaction> + Send + Sync + Clone,
{
    async fn execute(&self) -> Result<StepOutput, StepError> {
        let pending_tx = self
            .signer_middleware
            .send_transaction(self.tx_request.clone(), self.block)
            .await
            .map_err(|e| StepError::SendTransactionError(e.to_string()))?;

        let tx_receipt = pending_tx
            .await
            .map_err(|e| StepError::SendTransactionError(e.to_string()))?;
        Ok(SendTransactionOutput::TransactionReceipt(tx_receipt).into())
    }
}

#[derive(Debug, Clone)]
pub enum SendTransactionOutput {
    TransactionReceipt(Option<TransactionReceipt>),
}

#[derive(Debug, Error)]
pub enum SendTransactionError<M, S>
where
    M: Middleware,
    S: Signer,
{
    #[error(transparent)]
    SignerMiddlewareError(#[from] SignerMiddlewareError<M, S>),
    #[error(transparent)]
    ProviderError(#[from] ProviderError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::*;
    use ethers::prelude::*;
    use ethers::utils::Anvil;

    #[tokio::test]
    #[ignore]
    async fn can_send_transaction() {
        let anvil = Anvil::new().spawn();

        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let wallet2: LocalWallet = anvil.keys()[1].clone().into();

        // connect to the network
        let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();

        // connect the wallet to the provider
        let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));

        // craft the transaction
        let tx = TransactionRequest::new().to(wallet2.address()).value(10000);

        let step = SendTransactionBuilder::default()
            .signer_middleware(client)
            .tx_request(tx)
            .build()
            .unwrap();

        let output = step.execute().await.unwrap();
        let SendTransactionOutput::TransactionReceipt(tx_receipt) =
            output.unwrap_send_transaction_output();
        let _tx_receipt = assert_some!(tx_receipt);
    }
}
