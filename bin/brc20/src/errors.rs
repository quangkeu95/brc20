use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitcoinRpcError {
    #[error(transparent)]
    BitcoinCoreError(#[from] bitcoincore_rpc::Error),
    #[error(transparent)]
    HttpRequestError(#[from] reqwest::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
