use crate::errors::BitcoinRpcError;
use bitcoincore_rpc::bitcoincore_rpc_json::{GetBlockStatsResult, GetBlockchainInfoResult};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

mod types;
pub use types::*;

#[derive(Clone)]
pub struct BitcoinClient {
    pub rpc_url: String,
}

#[async_trait::async_trait]
pub trait BitcoinRpcClient: Clone + Send + Sync + 'static {
    async fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, BitcoinRpcError>;
    async fn get_block_stats(&self, height: u64) -> Result<GetBlockStatsResult, BitcoinRpcError>;
    async fn get_recommend_fee(&self) -> Result<RecommendedFee, BitcoinRpcError>;
}

impl BitcoinClient {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_owned(),
        }
    }
}

#[async_trait::async_trait]
impl BitcoinRpcClient for BitcoinClient {
    async fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResult, BitcoinRpcError> {
        let client = reqwest::Client::new();

        let response = client
            .post(&self.rpc_url)
            .header("Content-Type", "application/json")
            .body(
                r#"{
            "method": "getblockchaininfo"
        }"#,
            )
            .send()
            .await?
            .json::<BitcoinRpcResponse<GetBlockchainInfoResult>>()
            .await?;

        Ok(response.result)
    }

    async fn get_block_stats(&self, height: u64) -> Result<GetBlockStatsResult, BitcoinRpcError> {
        let client = reqwest::Client::new();

        let body = json!({
            "method": "getblockstats",
            "params": vec![height]
        });

        let response = client
            .post(&self.rpc_url)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?
            .json::<BitcoinRpcResponse<GetBlockStatsResult>>()
            .await?;

        Ok(response.result)
    }

    async fn get_recommend_fee(&self) -> Result<RecommendedFee, BitcoinRpcError> {
        let client = reqwest::Client::new();

        let response = client
            .get("https://mempool.space/api/v1/fees/recommended")
            .send()
            .await?
            .json::<RecommendedFee>()
            .await?;

        Ok(response)
    }
}
