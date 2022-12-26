use std::{collections::HashMap, time::Instant};

use binance_p2p_parser::{env::get_opts, parser::BotError, parser::Parser, tg};
use tg::Telegram;
use tokio;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    let opts = get_opts()?;
    let mut notified: HashMap<String, Instant> = HashMap::new();
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
                let now = Instant::now();
                if !notified.contains_key(&row.adv.adv_no)
                    || now.duration_since(notified[&row.adv.adv_no]).as_secs() > 60 * 60 * 24
                {
                    notified.insert(row.adv.adv_no, now);
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
