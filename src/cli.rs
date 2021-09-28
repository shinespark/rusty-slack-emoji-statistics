use crate::slack::SlackRequest;

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

    dbg!(channels.len());
    dbg!(channels);

    // TODO: channelに絞り込む？
    // TODO: channelにpost取りにいくメソッド生やす？
    println!("{:?}", channels.len());
    println!("{:?}", channels);

    Ok(())
}
