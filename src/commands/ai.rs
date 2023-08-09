use crate::settings::SETTINGS;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

#[command]
async fn ai(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // send the prompt
    let owned_args = args.to_owned();
    openai::set_key(SETTINGS.clone().openai);
    let prompt = openai::chat::ChatCompletionMessage {
        role: openai::chat::ChatCompletionMessageRole::User,
        content: Some(owned_args.message().to_string()),
        name: Some(msg.author.name.clone()),
        function_call: None,
    };
    let response = openai::chat::ChatCompletion::builder("gpt-3.5-turbo", vec![prompt])
        .create()
        .await?;
    let message = response.choices.first().unwrap().message.clone();
    // format message
    let discord_message = MessageBuilder::new()
        .push(message.content.clone().unwrap().trim())
        .build();
    msg.channel_id.say(&ctx, discord_message).await?;
    Ok(())
}
