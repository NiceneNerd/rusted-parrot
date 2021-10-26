use serde::{Deserialize, Serialize};

const CONF_FILENAME: &str = "config.json";
const QUOTE_FILENAME: &str = "quotes.txt";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
    pub interval_hours: u64,
}

fn main() {
    let conf_path: std::path::PathBuf = dirs2::config_dir()
        .expect("Failed to get config dir!")
        .join("rusted-parrot");

    let config: Config = serde_json::from_str(
        &std::fs::read_to_string(&conf_path.join(CONF_FILENAME))
            .expect("Could not open config file"),
    )
    .expect("Could not parse config file");

    let quotes: Vec<String> = std::fs::read_to_string(&conf_path.join(QUOTE_FILENAME))
        .expect("Cannot find quote source file")
        .lines()
        .map(|s| s.to_owned())
        .collect();

    let twitter_bot = rust_twitter_bot_lib::TwitterBot::new()
        .consumer_key(&config.consumer_key)
        .consumer_secret_key(&config.consumer_secret)
        .access_token(&config.access_key)
        .secret_access_token(&config.access_secret);

    for q in quotes {
        match twitter_bot.tweet(&q, None) {
            Ok(_) => println!("Tweeted \"{}\"", q),
            Err(e) => panic!("{:?}", e),
        };

        std::thread::sleep(std::time::Duration::from_millis(
            1000 * 60 * 60 * config.interval_hours,
        ));
    }
}
