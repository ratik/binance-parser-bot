use std::env;

use crate::parser::{BotError, BotOpts};

fn get_env(name: &str) -> Result<String, BotError> {
    let value = env::var_os(name)
        .ok_or(BotError::EnvError(format!(
            "Please provide {} env var",
            name
        )))?
        .into_string()
        .map_err(|_| BotError::EnvError(format!("Can not convert {} env var to string", name)))?;
    Ok(value)
}

pub fn get_opts() -> Result<BotOpts, BotError> {
    // get env asset
    let asset = get_env("ASSET")?;
    let pay_types = get_env("PAY_TYPES")?;
    let fiat = get_env("FIAT")?;
    let merchant_check = get_env("MERCHANT_CHECK").unwrap_or("0".to_string());
    let merchant_check = merchant_check == "1";
    let pay_types: Vec<String> = pay_types.split(",").map(|s| s.trim().to_string()).collect();
    let min_value = get_env("MIN_VALUE")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0.0);
    let tg_token = get_env("TG_TOKEN")?;
    let tg_channel_id = get_env("TG_CHANNEL_ID")?;
    Ok(BotOpts {
        asset,
        pay_types,
        fiat,
        merchant_check,
        min_value,
        tg_channel_id,
        tg_token,
    })
}
