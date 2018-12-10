use crate::settings::SETTINGS;
use chrono::DateTime;
use lazy_static::lazy_static;
use log::debug;
use serde_derive::Deserialize;
use serenity::command;
use serenity::utils::Colour;
use std::sync::RwLock;
use twapi::{Twapi, UserAuth};

lazy_static! {
    pub static ref TWITTER: RwLock<UserAuth> = RwLock::new({
        let settings = SETTINGS.read().expect("Settings");
        let twitter = UserAuth::new(
            settings.twitter.consumer_api_key.as_str(),
            settings.twitter.consumer_api_secret.as_str(),
            settings.twitter.access_token.as_str(),
            settings.twitter.access_token_secret.as_str(),
        );
        let params: Vec<(&str, &str)> = vec![
            ("include_entities", "false"),
            ("skip_status", "true"),
            ("include_email", "false"),
        ];
        if let Err(_) = twitter.get_verify_credentials(&params) {
            panic!("Couldn't verify Twitter credentials");
        }
        twitter
    });
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Timeline {
    Tweet(Vec<Tweet>),
}

#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
    pub user: TwitterUser,
}

#[derive(Debug, Deserialize)]
pub struct TwitterUser {
    pub screen_name: String,
    pub profile_image_url_https: String,
}

command!(command(_context, message, args) {
    // num of tweets to get, but limit to 5
    let count = match args.current() {
        Some(count) if count.parse::<u32>().expect("Parsing count") < 5 => count,
        Some(_) => {
            message.reply("The limit is 5. #MAGA")?;
            "5"
        },
        None => "1",
    };

    // get tweets
    let params: Vec<(&str, &str)> = vec![
        ("screen_name", "realDonaldTrump"),
        ("count", count),
        ("trim_user", "false"),
        ("exclude_replies", "false"),
        ("include_rts", "true"),
    ];
    let twitter = TWITTER.read().expect("Settings");
    match twitter.get(
        "https://api.twitter.com/1.1/statuses/user_timeline.json",
        &params,
    ) {
        Ok(mut response) => {
            // convert response to struct
            let timeline: Timeline = match response.json() {
                Ok(json) => json,
                Err(e) => {
                    debug!("Error: {:#?}", e);
                    message.reply("There was an issue with the response from the REPL.")?;
                    return Ok(());
                }
            };
            debug!("Twitter response: {:#?}", timeline);
            let Timeline::Tweet(tweets) = timeline;

            // iterate over the tweets and post them as an embed
            for tweet in tweets {
                let timestamp = DateTime::parse_from_str(
                    &tweet.created_at, "%a %h %d %H:%M:%S %z %Y"
                ).expect("Parsed timestamp").to_rfc3339();

                message.channel_id
                    .send_message(|m| {
                        m.embed(|e| {
                            e.timestamp(timestamp).author(|a| {
                                a.icon_url(&tweet.user.profile_image_url_https)
                                    .name(&format!("@{} - #MAGA Tweet", tweet.user.screen_name))
                                    .url(&format!("https://twitter.com/{}", tweet.user.screen_name))

                            }).description(tweet.text).colour(Colour::BLUE)
                        })
                    })?;
            }
        }
        Err(e) => {
            debug!("Error: {:#?}", e);
            message.reply("Sorry, there was an issue getting the tweet(s). #MAGA")?;
        }
    };
});
