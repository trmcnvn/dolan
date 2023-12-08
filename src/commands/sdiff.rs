use crate::settings::SETTINGS;
use serde::Serialize;
use serenity::builder::{CreateAttachment, CreateMessage};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[derive(Debug, Serialize)]
struct Request<'a> {
    prompt: &'a str,
}

#[command]
async fn sdiff(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // send the prompt
    let owned_args = args.to_owned();
    let client = reqwest::Client::new();
    let settings = SETTINGS.clone();
    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/ai/run/{}",
        settings.cf_account, "@cf/stabilityai/stable-diffusion-xl-base-1.0"
    );
    let request = Request {
        prompt: owned_args.message(),
    };
    let bytes = client
        .post(&endpoint)
        .bearer_auth(settings.cf_api)
        .json(&request)
        .send()
        .await?
        .bytes()
        .await?;
    let attachments = [CreateAttachment::bytes(
        bytes.to_vec(),
        "ai_image_response.png",
    )];
    let builder = CreateMessage::new().content("Here is your image");
    msg.channel_id
        .send_files(&ctx, attachments, builder)
        .await?;
    Ok(())
}
