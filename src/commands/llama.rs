use crate::settings::SETTINGS;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[derive(Debug, Serialize)]
struct Request<'a> {
    messages: [LlamaMessage<'a>; 1],
}

#[derive(Debug, Serialize)]
struct LlamaMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct Response {
    result: LlamaResponse,
}

#[derive(Debug, Deserialize)]
struct LlamaResponse {
    response: String,
}

#[command]
async fn llama(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // send the prompt
    let owned_args = args.to_owned();
    let client = reqwest::Client::new();
    let settings = SETTINGS.clone();
    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
        settings.cf_account, "@cf/meta/llama-2-7b-chat-int8"
    );
    let messages = [LlamaMessage {
        role: "user",
        content: owned_args.message(),
    }];
    let request = Request { messages };
    let response: Response = client
        .post(&endpoint)
        .bearer_auth(settings.cf_api)
        .json(&request)
        .send()
        .await?
        .json()
        .await?;
    let discord_message = MessageBuilder::new().push(response.result.response).build();
    msg.channel_id.say(&ctx, discord_message).await?;
    Ok(())
}
