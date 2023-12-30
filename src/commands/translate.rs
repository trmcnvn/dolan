use crate::settings::SETTINGS;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[derive(Debug, Serialize)]
struct Request<'a> {
    text: &'a str,
    source_lang: &'a str,
    target_lang: &'a str,
}

#[derive(Debug, Deserialize)]
struct Response {
    result: TranslationResponse,
}

#[derive(Debug, Deserialize)]
struct TranslationResponse {
    translated_text: String,
}

#[command]
async fn translate(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut owned_args = args.to_owned();
    let client = reqwest::Client::new();
    let settings = SETTINGS.clone();
    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
        settings.cf_account, "@cf/meta/m2m100-1.2b"
    );

    let source_lang = owned_args.single::<String>().unwrap_or("english".into());
    let target_lang = owned_args.single::<String>().unwrap_or("japanese".into());
    let text = owned_args.rest();
    let request = Request {
        text,
        source_lang: &source_lang,
        target_lang: &target_lang,
    };
    let response = client
        .post(&endpoint)
        .bearer_auth(settings.cf_api)
        .json(&request);
    let response = response.send().await?;
    let response: Response = response.json().await?;
    let discord_message = MessageBuilder::new()
        .push(response.result.translated_text)
        .build();
    msg.channel_id.say(&ctx, discord_message).await?;
    Ok(())
}
