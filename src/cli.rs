use crate::slack::SlackRequest;
use tokio_stream::{self, StreamExt};

pub async fn statistics(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let slack_request = SlackRequest::new(token);

    let emojis = slack_request.emoji_list().await?;

    dbg!(emojis.len());
    dbg!(emojis);

    let raw_channels = slack_request.channel_list().await?;
    let channels = raw_channels
        .into_iter()
        .filter(|c| c.is_channel == true)
        .collect::<Vec<_>>();

    let mut stream = tokio_stream::iter(&channels);
    while let Some(c) = stream.next().await {
        dbg!(&c.id);
        dbg!(&c.name);
        slack_request.channel_history_all(&c.id).await;
    }

    Ok(())
}
