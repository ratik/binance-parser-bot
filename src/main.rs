use binance_p2p_parser::{env::get_opts, parser::BotError, parser::Parser, tg};
use std::time::Duration;
use tg::Telegram;
use tokio;
use ttlhashmap::TtlHashMap;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    let opts = get_opts()?;
    let mut notified: TtlHashMap<String, bool> = TtlHashMap::new(Duration::from_secs(60 * 60 * 24));
    notified.autoclean = ttlhashmap::AutoClean::Modify;
    let parser = Parser { opts: &opts };
    let tg_api = Telegram {
        token: &opts.tg_token,
        chat_id: &opts.tg_channel_id,
    };

    loop {
        let rows = parser.get_rows().await?;
        for row in rows.data {
            let price = row
                .adv
                .price
                .parse::<f32>()
                .map_err(|e| BotError::ValueError(e.to_string()))?;
            if price > opts.min_value {
                let key = row.adv.adv_no.clone() + &row.adv.price;
                if !notified.contains_key(&key) {
                    notified.insert(key, true);
                    let message = format!(
                        "New offer: {} {} for {} {} by {}",
                        row.adv.price,
                        opts.fiat,
                        row.adv.price,
                        opts.asset,
                        row.advertiser.nick_name
                    );
                    tg_api.send_message(message).await?;
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
