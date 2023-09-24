use crate::constants::MAX_NUMBER_OF_TRIES;
use serde_json::json;
use std::error::Error;

pub type Message = String;

pub struct TelegramNotifier {
    token: String,
    chat_id: String,
}

impl TelegramNotifier {
    pub fn new(token: String, chat_id: String) -> TelegramNotifier {
        TelegramNotifier { token, chat_id }
    }

    pub fn send_message(&self, message: &str) -> Result<(), Box<dyn Error>> {
        tracing::info!("sending message to Telegram about new ads");
        let client = reqwest::blocking::Client::new();

        let mut number_of_tries = MAX_NUMBER_OF_TRIES;

        loop {
            let result = client
                .post(format!(
                    "https://api.telegram.org/bot{token}/sendMessage",
                    token = &self.token
                ))
                .json(&json!({
                    "text": message,
                    "chat_id": self.chat_id
                }))
                .send();

            match result {
                Ok(_) => {
                    tracing::info!("message is successfully sent to Telegram");
                    break;
                }
                Err(err) => {
                    tracing::error!("failed to send message to Telegram: {:?}", err);
                    number_of_tries -= 1;
                    tracing::info!("number of tries left: {}", number_of_tries);
                    if number_of_tries == 0 {
                        tracing::error!("number of tries exceeded");
                        return Err(Box::new(err));
                    }
                }
            }
        }

        Ok(())
    }
}
