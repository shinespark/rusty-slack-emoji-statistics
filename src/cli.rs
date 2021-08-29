use crate::slack::SlackRequest;

pub async fn statistics(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let slack_request = SlackRequest::new(token);

    let emojis = slack_request.list_emoji().await?;

    println!("{:?}", emojis);
    println!("{:?}", emojis.len());

    Ok(())
}
