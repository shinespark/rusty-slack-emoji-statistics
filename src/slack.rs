use reqwest::{header, Client, StatusCode};

const EMOJI_LIST_API: &str = "https://slack.com/api/emoji.list";

pub struct SlackRequest {
    token: String,
}

impl SlackRequest {
    pub fn new(token: &str) -> Self {
        Self {
            token: format!("Bearer {}", token),
        }
    }

    pub async fn lists_custom_emoji(
        &self,
    ) -> Result<(StatusCode, String), Box<dyn std::error::Error>> {
        let client = Client::builder().build()?;
        let res = client
            .get(EMOJI_LIST_API)
            .header(header::AUTHORIZATION, &self.token)
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .send()
            .await?;

        let status_code = res.status();
        let body = res.text().await?;
        Ok((status_code, body))
    }
}
