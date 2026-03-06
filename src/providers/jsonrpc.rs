use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;

use crate::client::{JsonRpcRequest, JsonRpcResponse, RpcConfig};
use crate::error::{Result, RpcError};
use crate::traits::BlockchainClient;
use crate::types::*;

#[derive(Clone)]
pub struct JsonRpcClient {
    config: Arc<RpcConfig>,
    http_client: Client,
    chain: Chain,
    network: Network,
}

impl JsonRpcClient {
    pub fn new(config: RpcConfig, chain: Chain, network: Network) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(config.timeout())
            .build()?;

        Ok(Self {
            config: Arc::new(config),
            http_client,
            chain,
            network,
        })
    }

    pub fn chain(&self) -> Chain {
        self.chain
    }

    pub fn network(&self) -> Network {
        self.network
    }

    async fn call<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let request = JsonRpcRequest::new(method.to_string(), params);

        let response = self
            .http_client
            .post(&self.config.url)
            .basic_auth(&self.config.username, Some(&self.config.password))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(RpcError::RequestFailed(format!(
                "http {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let rpc_response: JsonRpcResponse<T> = response.json().await?;

        if let Some(error) = rpc_response.error {
            return Err(RpcError::RpcResponseError {
                code: error.code,
                message: error.message,
            });
        }

        rpc_response
            .result
            .ok_or_else(|| RpcError::InvalidResponse("missing result field".to_string()))
    }
}

#[async_trait]
impl BlockchainClient for JsonRpcClient {
    async fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.call("getblockchaininfo", json!([])).await
    }

    async fn get_raw_transaction_with_block(
        &self,
        txid: &str,
        verbose: bool,
        blockhash: Option<&str>,
    ) -> Result<RawTransaction> {
        let params = if let Some(hash) = blockhash {
            json!([txid, verbose, hash])
        } else {
            json!([txid, verbose])
        };

        let result: std::result::Result<RawTransaction, RpcError> =
            self.call("getrawtransaction", params).await;

        if result.is_err() {
            let wallet_tx: serde_json::Value = self.call("gettransaction", json!([txid])).await?;

            if let Some(hex) = wallet_tx.get("hex").and_then(|h| h.as_str()) {
                return self.call("decoderawtransaction", json!([hex])).await;
            }
        }

        result
    }

    async fn list_unspent(
        &self,
        min_conf: Option<u32>,
        max_conf: Option<u32>,
        addresses: Option<&[String]>,
    ) -> Result<Vec<Utxo>> {
        let params = json!([
            min_conf.unwrap_or(1),
            max_conf.unwrap_or(9_999_999),
            addresses.unwrap_or(&[])
        ]);

        self.call("listunspent", params).await
    }

    async fn list_transactions(
        &self,
        label: Option<&str>,
        count: Option<usize>,
        skip: Option<usize>,
        include_watchonly: bool,
    ) -> Result<Vec<TransactionListItem>> {
        let params = json!([
            label.unwrap_or("*"),
            count.unwrap_or(10),
            skip.unwrap_or(0),
            include_watchonly
        ]);

        self.call("listtransactions", params).await
    }

    async fn get_received_by_address(&self, address: &str, min_conf: Option<u32>) -> Result<f64> {
        let params = json!([address, min_conf.unwrap_or(1)]);
        self.call("getreceivedbyaddress", params).await
    }

    async fn list_received_by_address(
        &self,
        min_conf: Option<u32>,
        include_empty: bool,
        include_watchonly: bool,
    ) -> Result<Vec<ReceivedByAddress>> {
        let params = json!([min_conf.unwrap_or(1), include_empty, include_watchonly]);
        self.call("listreceivedbyaddress", params).await
    }

    async fn is_address_watched(&self, address: &str) -> Result<bool> {
        let validation = self.validate_address(address).await?;

        if !validation.isvalid {
            return Ok(false);
        }

        if let Some(is_watchonly) = validation.iswatchonly {
            return Ok(is_watchonly);
        }

        let received = self
            .list_received_by_address(Some(0), true, true)
            .await?;

        Ok(received.iter().any(|r| r.address == address))
    }

    async fn import_address(
        &self,
        address: &str,
        label: Option<&str>,
        rescan: bool,
    ) -> Result<()> {
        let params = json!([address, label.unwrap_or(""), rescan]);
        self.call::<serde_json::Value>("importaddress", params)
            .await?;
        Ok(())
    }

    async fn validate_address(&self, address: &str) -> Result<AddressValidation> {
        self.call("validateaddress", json!([address])).await
    }

    async fn get_transaction(&self, txid: &str) -> Result<serde_json::Value> {
        self.call("gettransaction", json!([txid])).await
    }

    async fn get_block_count(&self) -> Result<u64> {
        self.call("getblockcount", json!([])).await
    }

    async fn get_best_block_hash(&self) -> Result<String> {
        self.call("getbestblockhash", json!([])).await
    }
}
