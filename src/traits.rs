use async_trait::async_trait;

use crate::error::Result;
use crate::types::{
    AddressValidation, BlockchainInfo, RawTransaction, ReceivedByAddress, TransactionListItem, Utxo,
};

#[async_trait]
pub trait BlockchainClient: Send + Sync {
    async fn get_blockchain_info(&self) -> Result<BlockchainInfo>;

    async fn get_raw_transaction(&self, txid: &str, verbose: bool) -> Result<RawTransaction> {
        self.get_raw_transaction_with_block(txid, verbose, None)
            .await
    }

    async fn get_raw_transaction_with_block(
        &self,
        txid: &str,
        verbose: bool,
        blockhash: Option<&str>,
    ) -> Result<RawTransaction>;

    async fn list_unspent(
        &self,
        min_conf: Option<u32>,
        max_conf: Option<u32>,
        addresses: Option<&[String]>,
    ) -> Result<Vec<Utxo>>;

    async fn list_transactions(
        &self,
        label: Option<&str>,
        count: Option<usize>,
        skip: Option<usize>,
        include_watchonly: bool,
    ) -> Result<Vec<TransactionListItem>>;

    async fn get_received_by_address(&self, address: &str, min_conf: Option<u32>) -> Result<f64>;

    async fn list_received_by_address(
        &self,
        min_conf: Option<u32>,
        include_empty: bool,
        include_watchonly: bool,
    ) -> Result<Vec<ReceivedByAddress>>;

    async fn is_address_watched(&self, address: &str) -> Result<bool>;

    async fn import_address(&self, address: &str, label: Option<&str>, rescan: bool)
        -> Result<()>;

    async fn validate_address(&self, address: &str) -> Result<AddressValidation>;

    async fn get_transaction(&self, txid: &str) -> Result<serde_json::Value>;

    async fn get_block_count(&self) -> Result<u64>;

    async fn get_best_block_hash(&self) -> Result<String>;
}

#[derive(Debug, Clone, Copy)]
pub struct PaginationParams {
    pub skip: usize,
    pub count: usize,
}

#[async_trait]
pub trait PaginatedBlockchainClient: BlockchainClient {
    async fn list_transactions_paginated(
        &self,
        params: PaginationParams,
        include_watchonly: bool,
    ) -> Result<Vec<TransactionListItem>> {
        let mut all = Vec::new();
        let mut skip = params.skip;
        let count = params.count;

        loop {
            let batch = self
                .list_transactions(None, Some(count), Some(skip), include_watchonly)
                .await?;

            let batch_len = batch.len();
            if batch_len == 0 {
                break;
            }

            all.extend(batch);
            skip += batch_len;

            if batch_len < count {
                break;
            }
        }

        Ok(all)
    }
}

impl<T> PaginatedBlockchainClient for T where T: BlockchainClient + ?Sized {}
