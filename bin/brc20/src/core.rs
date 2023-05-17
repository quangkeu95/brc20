use crate::bitcoin_client::{BitcoinRpcClient, RecommendedFee};
use async_shutdown::Shutdown;
use bitcoincore_rpc::bitcoincore_rpc_json::{GetBlockStatsResult, GetBlockchainInfoResult};
use paris::info;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::time;

pub async fn spawn_watch_block_interval<T>(
    client: T,
    interval: Duration,
    sender: Sender<GetBlockchainInfoResult>,
    shutdown: Shutdown,
) where
    T: BitcoinRpcClient,
{
    tokio::spawn({
        let client = client.clone();
        let mut interval_task = time::interval(interval);
        let sender = sender.clone();
        let shutdown = shutdown.clone();

        async move {
            let mut latest_block = 0;

            loop {
                tokio::select! {
                    _ = shutdown.wait_shutdown_triggered() => {
                        info!("Receive shutdown signal. Exiting block watcher...");
                        return;
                    }
                     _ = interval_task.tick() => {
                        if let Ok(block_info) = client.get_blockchain_info().await {
                            if latest_block == 0 {
                                latest_block = block_info.blocks;
                            } else if block_info.blocks <= latest_block {
                                continue;
                            }

                            latest_block = block_info.blocks;
                            let _ = sender.send(block_info);
                        }

                    }
                }
            }
        }
    });

    info!(
        "Block watcher spawned with interval <cyan>{:?}</>",
        interval
    );
}

pub async fn spawn_watch_block_stats<T>(
    client: T,
    interval: Duration,
    mut block_rx: Receiver<GetBlockchainInfoResult>,
    sender: Sender<GetBlockStatsResult>,
    shutdown: Shutdown,
) where
    T: BitcoinRpcClient,
{
    // Watch block stats with interval
    tokio::spawn({
        let client = client.clone();
        let mut interval_task = time::interval(interval);

        async move {
            let mut block_stats: Option<GetBlockStatsResult> = None;
            let mut latest_block_height: Option<u64> = None;

            loop {
                tokio::select! {
                    _ = shutdown.wait_shutdown_triggered() => {
                    info!("Receive shutdown signal. Exiting block stats watcher...");
                        return;
                }
                    _ = interval_task.tick() => {
                        if let Some(latest_block_height) = latest_block_height {
                            if let Ok(block_stats) = client.get_block_stats(latest_block_height).await {
                                let _ = sender.send(block_stats);
                            }
                        }
                    }
                    block_info_res = block_rx.recv() => {
                        if let Ok(block_info) = block_info_res {
                            latest_block_height = Some(block_info.blocks);
                        }
                    }
                }
            }
        }
    });

    info!(
        "Block stats watcher spawned with interval <cyan>{:?}</>",
        interval
    );
}

pub async fn spawn_watch_recommended_fee<T>(
    client: T,
    interval: Duration,
    sender: tokio::sync::mpsc::UnboundedSender<RecommendedFee>,
    shutdown: Shutdown,
) where
    T: BitcoinRpcClient,
{
    tokio::spawn({
        let client = client.clone();
        let mut interval_task = time::interval(interval);

        async move {
            loop {
                tokio::select! {
                    _ = shutdown.wait_shutdown_triggered() => {
                    info!("Receive shutdown signal. Exiting recommended fee watcher...");
                        return;
                }
                    _ = interval_task.tick() => {
                            if let Ok(fee_stats) = client.get_recommend_fee().await {
                                let _ = sender.send(fee_stats);
                            }
                    }

                }
            }
        }
    });

    info!("Fee watcher spawned with interval <cyan>{:?}</>", interval);
}
