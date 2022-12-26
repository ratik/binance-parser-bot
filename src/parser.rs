use reqwest::{Client, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug)]
pub enum BotError {
    ValueError(String),
    EnvError(String),
    NetworkError(reqwest::Error),
}

pub struct BotOpts {
    pub asset: String,
    pub pay_types: Vec<String>,
    pub fiat: String,
    pub merchant_check: bool,
    pub min_value: f32,
    pub tg_token: String,
    pub tg_channel_id: String,
}

pub struct Parser<'a> {
    pub opts: &'a BotOpts,
}

impl Parser<'_> {
    pub async fn get_rows(&self) -> Result<BinanceResponse, BotError> {
        let data = self.get_data().await?;
        let json_string = data
            .text()
            .await
            .map_err(|err| BotError::NetworkError(err))?;
        // unmarshal data
        let rows: BinanceResponse = serde_json::from_str(&json_string).unwrap();
        Ok(rows)
    }

    async fn get_data(&self) -> Result<Response, BotError> {
        let client = Client::new();
        let payload = json!({
            "proMerchantAds": false,
            "page": 1,
            "rows": 20,
            "payTypes": self.opts.pay_types,
            "countries": [],
            "publisherType": "merchant",
            "tradeType": "SELL",
            "asset": self.opts.asset,
            "fiat": self.opts.fiat,
            "merchantCheck": self.opts.merchant_check,
        });
        let resp = client
        .post("https://p2p.binance.com/bapi/c2c/v2/friendly/c2c/adv/search")
        .header("content-type", "application/json")
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .body(json!(payload).to_string())
        .send()
        .await.map_err(|e| BotError::NetworkError(e))?;
        Ok(resp)
    }
}

#[derive(Debug, Deserialize)]
pub struct BinanceResponse {
    pub code: String,
    pub total: u32,
    pub data: Vec<BinanceResponseRow>,
}

#[derive(Debug, Deserialize)]
pub struct BinanceResponseRow {
    pub adv: BinanceResponseRowAdv,
    pub advertiser: BinanceResponseRowAdvertiser,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceResponseRowAdv {
    pub adv_no: String,
    pub max_single_trans_amount: String,
    pub min_single_trans_amount: String,
    pub price: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceResponseRowAdvertiser {
    pub nick_name: String,
    pub user_identity: String,
}
