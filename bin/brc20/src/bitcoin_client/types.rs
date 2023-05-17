use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BitcoinRpcResponse<T> {
    pub result: T,
    pub error: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecommendedFee {
    #[serde(rename = "fastestFee")]
    pub fastest_fee: u64,
    #[serde(rename = "halfHourFee")]
    pub half_hour_fee: u64,
    #[serde(rename = "hourFee")]
    pub hour_fee: u64,
    #[serde(rename = "economyFee")]
    pub economy_fee: u64,
    #[serde(rename = "minimumFee")]
    pub minimum_fee: u64,
}
