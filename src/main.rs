use std::path::PathBuf;
use std::time;

mod config;
mod twitter;

const CONF_FILENAME: &str = ".twitter-bot.conf";
const QUOTE_FILENAME: &str = ".quotes.txt";

fn get_home_dir() -> PathBuf {
    match dirs2::config_dir() {
        Some(p) => p,
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

fn main() {
    let conf_path: PathBuf = get_home_dir();

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
        twitter.tweet(q);

        std::thread::sleep(time::Duration::from_millis(1000 * config.interval_sec));
    }
}
