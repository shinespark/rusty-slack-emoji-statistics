use reqwest::{header, Client};
use serde::Deserialize;
use std::collections::HashMap;

const CONTENT_TYPE: &str = "application/json; charset=utf-8";
const CONVERSATIONS_LIST_API: &str = "https://slack.com/api/conversations.list";
const CONVERSATIONS_HISTORY_API: &str = "https://slack.com/api/conversations.history";
const EMOJI_LIST_API: &str = "https://slack.com/api/emoji.list";

pub struct SlackRequest {
    token: String,
}

#[derive(Debug, Deserialize)]
struct ConversationsListResponse {
    ok: bool,
    channels: Vec<Channel>,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub is_channel: bool,
    is_group: bool,
    is_im: bool,
    created: u64,
    creator: String,
    is_archived: bool,
    is_general: bool,
    unlinked: u64,
    name_normalized: String,
    is_shared: bool,
    is_ext_shared: bool,
    is_org_shared: bool,
    is_pending_ext_shared: bool,
    is_member: bool,
    is_private: bool,
    is_mpim: bool,
    num_members: u64,
}

#[derive(Debug, Deserialize)]
struct ConversationsHistoryResponse {
    ok: bool,
    messages: Vec<Message>,
    has_more: bool,
    pin_count: u64,
    response_metadata: ResponseMetadata,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(rename(deserialize = "type"))]
    type_: String,
    user: String,
    text: String,
    ts: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetadata {
    next_cursor: String,
}

#[derive(Debug, Deserialize)]
struct EmojiListResponse {
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

    pub async fn channel_list(&self) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
        let client = Client::builder().build()?;
        let res = client
            .get(CONVERSATIONS_LIST_API)
            .header(header::AUTHORIZATION, &self.token)
            .header(header::CONTENT_TYPE, CONTENT_TYPE)
            .send()
            .await?;
        let body = res.text().await?;
        let json: ConversationsListResponse = serde_json::from_str(&body)?;

        Ok(json.channels)
    }

    pub async fn channel_history_all(
        &self,
        channel_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::builder().build()?;
        dbg!(channel_id);
        let messages = self
            .channel_history(client, channel_id, "".to_string())
            .await;

        Ok(())
    }

    async fn channel_history(
        &self,
        client: Client,
        channel_id: &str,
        _next_cursor: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        dbg!("aa");
        let query = &[("channel", channel_id)];

        let res = client
            .get(CONVERSATIONS_HISTORY_API)
            .header(header::AUTHORIZATION, &self.token)
            .header(header::CONTENT_TYPE, CONTENT_TYPE)
            .query(query)
            .send()
            .await?;
        let body = res.text().await?;
        let json: ConversationsHistoryResponse = serde_json::from_str(&body)?;

        dbg!(json);

        Ok(())
    }

    pub async fn emoji_list(&self) -> Result<Vec<Emoji>, Box<dyn std::error::Error>> {
        let client = Client::builder().build()?;
        let res = client
            .get(EMOJI_LIST_API)
            .header(header::AUTHORIZATION, &self.token)
            .header(header::CONTENT_TYPE, CONTENT_TYPE)
            .send()
            .await?;
        let body = res.text().await?;
        let json: EmojiListResponse = serde_json::from_str(&body)?;

        let emojis = json
            .emoji
            .into_iter()
            .map(|(k, v)| Emoji { name: k, url: v })
            .collect::<Vec<_>>();

        Ok(emojis)
    }
}
