use crate::Step;
use derive_builder::Builder;
use ethers::middleware::signer::SignerMiddlewareError;
use ethers::prelude::{Middleware, ProviderError, Signer, SignerMiddleware, TransactionReceipt};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::BlockId;
use ethers::types::TransactionRequest;
use thiserror::Error;

/// Allow to send transaction using a [`SignerMiddleware`]
#[derive(Debug)]
pub struct SendTransaction<M: Middleware, S: Signer> {
    pub signer_middleware: SignerMiddleware<M, S>,
}

impl<M, S> SendTransaction<M, S>
where
    M: Middleware,
    S: Signer,
{
    pub fn new(signer_middleware: SignerMiddleware<M, S>) -> Self {
        Self { signer_middleware }
    }
}

#[async_trait::async_trait]
impl<M, S> Step for SendTransaction<M, S>
where
    M: Middleware,
    S: Signer,
{
    type Input = SendTransactionInput<TransactionRequest>;

    type Output = SendTransactionOutput;

    type Error = SendTransactionError<M, S>;

    async fn execute(&self, input: &mut Self::Input) -> Result<Self::Output, Self::Error> {
        let pending_tx = self
            .signer_middleware
            .send_transaction(input.tx_request.clone(), input.block)
            .await?;

        let tx_receipt = pending_tx.await?;
        Ok(SendTransactionOutput::TransactionReceipt(tx_receipt))
    }
}

#[derive(Debug, Builder)]
pub struct SendTransactionInput<Tx>
where
    Tx: Into<TypedTransaction> + Send + Sync,
{
    pub tx_request: Tx,
    #[builder(default)]
    pub block: Option<BlockId>,
    // pub is_returning_pending_tx: bool,
}

#[derive(Debug)]
pub enum SendTransactionOutput {
    // PendingTx(PendingTransaction<'a, P>),
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

        let step = SendTransaction::new(client);

        let mut send_transaction_input = SendTransactionInputBuilder::default()
            .tx_request(tx)
            .build()
            .unwrap();

        let output = step.execute(&mut send_transaction_input).await.unwrap();
        let SendTransactionOutput::TransactionReceipt(tx_receipt) = output;
        let _tx_receipt = assert_some!(tx_receipt);
    }
}
