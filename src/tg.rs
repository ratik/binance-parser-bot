use reqwest::Client;
use serde_json::json;

use crate::parser::BotError;

pub struct Telegram<'a> {
    pub token: &'a String,
    pub chat_id: &'a String,
}

impl Telegram<'_> {
    pub async fn send_message(&self, message: String) -> Result<(), BotError> {
        let client = Client::new();
        let payload = json!({
            "chat_id": self.chat_id,
            "text": message,
        });
        let _resp = client
            .post(format!(
                "https://api.telegram.org/bot{}/sendMessage",
                self.token
            ))
            .header("content-type", "application/json")
            .body(payload.to_string())
            .send()
            .await
            .map_err(|e| BotError::NetworkError(e))?;
        Ok(())
    }
}
