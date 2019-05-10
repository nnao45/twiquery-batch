mod exec;

extern crate tweetust;
extern crate twitter_stream;
extern crate twitter_stream_message;
extern crate chrono;
extern crate snailquote;
extern crate curl;
extern crate slog_scope;

use curl::Error as CurlError;

use twitter_stream::{TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream_message::StreamMessage;

use chrono::Local;

use snailquote::unescape;
use exec as Exec;

use serde::Deserialize;

use slog::{slog_info, slog_error, slog_crit};

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
    track: String,
    slack_url: String,
    pub is_debug: bool,
}

impl Config {
    pub fn new() -> Result<Self, ()> {
        match envy::from_env::<Config>() {
            Ok(config) => Ok(config),
            Err(e) => panic!("{:#?}", e),
        }
    }
}

pub struct TwitterClient {
    pub config: Config,
}

impl TwitterClient {
    pub fn new(cfg: Config) -> Self {
        TwitterClient {
                config: cfg,
        }
    }

    pub fn watch(self) {
        let consumer_key: &str = &self.config.consumer_key;
        let consumer_secret: &str = &self.config.consumer_secret;
        let access_token: &str = &self.config.access_token;
        let access_token_secret: &str = &self.config.access_token_secret;
        let track: &str = &self.config.track;
        let bot = TwitterStreamBuilder::filter(twitter_stream::Token::new(
                    consumer_key,
                    consumer_secret,
                    access_token,
                    access_token_secret,
                ))
            .track(Some(track))
            .listen()
            .unwrap()
            .flatten_stream()
            .for_each(move |json| {
                if let Ok(StreamMessage::Tweet(tweet)) = StreamMessage::from_str(&json) {
                    match Exec::Executer::new(
                        &self.config.slack_url,
                        Exec::TweiqueryData::new(
                            &self.config.track,
                            &format!("{}", &tweet.user.name)[..],
                            &format!("{}", &tweet.user.screen_name)[..],
                            &unescape(&format!("{:?}", &tweet.text)).unwrap()[..],
                            &format!("{}",tweet.created_at.with_timezone(&Local))[..],
                            &format!("{}", &tweet.id)[..],
                        ),
                    )
                    .exec_console()
                    .exec_curl() {
                        Ok(()) => slog_info!(slog_scope::logger(), "Slack request done"),
                        CurlError => slog_error!(slog_scope::logger(), "Slack request may error occured: {:#?}", CurlError),
                    };
                }
                Ok(())
            })
            .map_err(|e| slog_error!(slog_scope::logger(), "error: {}", e));

        rt::run(bot);
    }
}