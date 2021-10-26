// use twitter_api::{self, oauth::Token};
use egg_mode::*;

pub struct Twitter {
    token: Token
}

impl Twitter{
    pub fn new (consumer_key : String, consumer_secret : String,
                access_key : String, access_secret : String) -> Self {
        Twitter {
            token: Token::Access {
                access: KeyPair::new(access_key, access_secret),
                consumer: KeyPair::new(consumer_key, consumer_secret)
            }
        }
    }

    pub async fn tweet (&self, message : String)  {
        tweet::DraftTweet::new(message).send(&self.token).await.unwrap();
    }
}

