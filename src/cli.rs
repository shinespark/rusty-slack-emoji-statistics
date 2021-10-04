use crate::slack::SlackRequest;
use tokio_stream::{self, StreamExt};

pub async fn statistics(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let slack_request = SlackRequest::new(token);

    // emojis
    let emojis = slack_request.emoji_list().await?;

    dbg!(emojis.len());
    dbg!(emojis);

    // channels
    let raw_channels = slack_request.channel_list().await?;
    let channels = raw_channels
        .into_iter()
        .filter(|c| c.is_channel == true && c.is_archived == false)
        .collect::<Vec<_>>();

    let mut stream = tokio_stream::iter(&channels);
    while let Some(c) = stream.next().await {
        dbg!(&c.name);
        slack_request.channel_join(&c.id).await?;
        slack_request.channel_history(&c.id).await;
    }

    // slack_request.channel_join("C06BS3370").await?;
    // let ch = slack_request.channel_history("C06BS3370").await?;

    Ok(())
}
