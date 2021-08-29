extern crate clap;

use clap::{App, Arg};

use rusty_slack_emoji_statistics::cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Rusty Slack Emoji Statistics")
        .version("0.99")
        .about("Slack Emoji Statistics")
        .arg(
            Arg::with_name("SLACK_TOKEN")
                .short("t")
                .long("token")
                .help(
                    "Sets a slack token. e.g.) xoxp-***********-************-************-********************************")
                .required(true)
                .takes_value(true)
        )
        .get_matches();

    let token = matches.value_of("SLACK_TOKEN").unwrap();
    let _ = cli::statistics(token).await;

    Ok(())
}
