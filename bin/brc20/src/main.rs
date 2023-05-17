use async_shutdown::Shutdown;
use bitcoincore_rpc::{
    bitcoincore_rpc_json::{GetBlockStatsResult, GetBlockchainInfoResult},
    Auth, Client, RpcApi,
};
use brc20::{
    bitcoin_client::{BitcoinClient, BitcoinRpcClient, RecommendedFee},
    spawn_watch_block_interval, spawn_watch_block_stats, spawn_watch_recommended_fee,
};
use lazy_static::lazy_static;
use paris::{error, info};
use std::env;
use std::{sync::Arc, time::Duration};

lazy_static! {
    static ref WATCH_BLOCK_INTERVAL: Duration = Duration::from_secs(10);
    static ref WATCH_FEE_INTERVAL: Duration = Duration::from_secs(5);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    info!("Starting app...");

    let rpc_url = env::var("BITCOIN_RPC_URL").expect("BITCOIN_RPC_URL env not found");

    let client = BitcoinClient::new(&rpc_url);
    info!("Bitcoin RPC client initialized!");

    let shutdown = Shutdown::new();

    let (block_sender, mut block_rx) =
        tokio::sync::broadcast::channel::<GetBlockchainInfoResult>(5);

    let (fee_watcher_sender, mut fee_watcher_rx) =
        tokio::sync::mpsc::unbounded_channel::<RecommendedFee>();

    spawn_watch_block_interval(
        client.clone(),
        *WATCH_BLOCK_INTERVAL,
        block_sender.clone(),
        shutdown.clone(),
    )
    .await;

    spawn_watch_recommended_fee(
        client.clone(),
        *WATCH_FEE_INTERVAL,
        fee_watcher_sender,
        shutdown.clone(),
    )
    .await;

    tokio::spawn({
        async move {
            while let Some(fee) = fee_watcher_rx.recv().await {
                info!("────────────────────────────────────────────────────────────");
                info!("Hour fee (slow) <yellow>{:?}</> ", fee.hour_fee);
                info!("Half hour fee (normal) <cyan>{:?}</> ", fee.half_hour_fee);
                info!("Fast fee (fast) <red>{:?}</> ", fee.fastest_fee);
                info!("────────────────────────────────────────────────────────────");
            }
        }
    });

    tokio::spawn({
        async move {
            while let Ok(block_info) = block_rx.recv().await {
                info!("Receive new block <yellow>{:?}</>", block_info.blocks);
            }
        }
    });

    if let Err(e) = tokio::signal::ctrl_c().await {
        error!("Failed to wait for CTRL+C: {}", e);
        std::process::exit(1);
    } else {
        info!("Received interrupt signal. Shutting down server...");
        shutdown.shutdown();
    }

    Ok(())
}
