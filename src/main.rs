use std::path::PathBuf;
use std::time;

mod config;
mod twitter;

const CONF_FILENAME: &str = "config.json";
const QUOTE_FILENAME: &str = "quotes.txt";

#[tokio::main]
async fn main() {
    let conf_path: PathBuf = dirs2::config_dir().expect("Failed to get config dir!").join("twitter-bot");

    let config =
        config::Config::read(&conf_path.join(CONF_FILENAME)).expect("Cannot find config file");

    let quotes: Vec<String> = std::fs::read_to_string(&conf_path.join(QUOTE_FILENAME))
        .expect("Cannot find quote source file")
        .lines()
        .map(|s| s.to_owned())
        .collect();
    let twitter = twitter::Twitter::new(
        config.consumer_key,
        config.consumer_secret,
        config.access_key,
        config.access_secret,
    );

    for q in quotes {
        futures::executor::block_on(twitter.tweet(q));

        std::thread::sleep(time::Duration::from_millis(1000 * 60 * 60 * config.interval_hours));
    }
}
