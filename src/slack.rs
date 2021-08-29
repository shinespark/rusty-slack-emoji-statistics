use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashMap;

const EMOJI_LIST_API: &str = "https://slack.com/api/emoji.list";

pub struct SlackRequest {
    token: String,
}

#[derive(Debug, Deserialize)]
struct ListEmojiResponse {
    ok: bool,
    emoji: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Emoji {
    name: String,
    url: String,
}

impl SlackRequest {
    pub fn new(token: &str) -> Self {
        Self {
            token: format!("Bearer {}", token),
        }
    }

    pub async fn list_emoji(&self) -> Result<Vec<Emoji>, Box<dyn std::error::Error>> {
        let client = Client::builder().build()?;
        let res = client
            .get(EMOJI_LIST_API)
            .header(header::AUTHORIZATION, &self.token)
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .send()
            .await?;
        let body = res.text().await?;
        let json: ListEmojiResponse = serde_json::from_str(&body)?;

        let emojis = json
            .emoji
            .into_iter()
            .map(|(k, v)| Emoji { name: k, url: v })
            .collect::<Vec<_>>();

        Ok(emojis)
    }
}
