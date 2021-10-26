# Rusted Parrot

A simple Twitter bot written in Rust that tweets from a text file one line at a
time at regular intervals. The main purpose is for quote bots, but it can really
be used for anything with a static list of tweets.

## Usage 

1. Create a config folder
   - `~/.config/rusted-parrot` on Linux
   - `%APPDATA%\rusted-parrot` on Windows
2. Create the config file using the template below, and save it as
   `config.json` in the config folder you created.
3. Create your tweet text file, one tweet/quote per line, and save it as 
   `quotes.txt` in the config folder.
4. Run `rusted-parrot`, (or `cargo run` if building from source)

## Config File

```
{
  "consumer_key": "<consumer-key>",
  "consumer_secret": "<consumer-secret>",
  "access_key": "<access-key>",
  "access_secret": "<access-secret>",
  "interval_hours" : <num of hours between tweets>
}
```

## License

This project is licensed under the GPLv3 or later. It was originally based on
`twitter-bot-rs` at [https://github.com/juli1/twitter-bot-rs], but that project
seems to be abandoned with no license and in any case very little of the original
code remains in this project.