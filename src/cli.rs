use crate::slack::SlackRequest;

pub async fn statistics(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let slack_request = SlackRequest::new(token);

    let emojis = slack_request.emoji_list().await?;

    println!("{:?}", emojis.len());
    println!("{:?}", emojis);

    let channels = slack_request.channel_list().await?;

    // TODO: channelに絞り込む？
    // TODO: channelにpost取りにいくメソッド生やす？
    println!("{:?}", channels.len());
    println!("{:?}", channels);

    Ok(())
}
