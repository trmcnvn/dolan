use crate::settings::SETTINGS;
use openai::chat::{
    ChatCompletion, ChatCompletionFunctionDefinition, ChatCompletionMessage,
    ChatCompletionMessageRole,
};
use openai::OpenAiError;
use serde_json::json;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::sync::LazyLock;

use super::weather::get_weather_json;

type FunctionVec = [ChatCompletionFunctionDefinition; 1];
static FUNCTIONS: LazyLock<FunctionVec> = LazyLock::new(|| {
    [ChatCompletionFunctionDefinition {
        name: "get_weather".into(),
        description: Some("Get the current weather in a given location".into()),
        parameters: Some(json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The location to get weather for"
                }
            },
            "required": ["location"]
        })),
    }]
});

async fn function_gpt(
    user: String,
    messages: &[ChatCompletionMessage],
) -> Result<String, OpenAiError> {
    let response = ChatCompletion::builder("gpt-3.5-turbo", messages)
        .user(user)
        .create()
        .await?;
    let response = response.choices.first().unwrap().message.clone();
    let response = response.content.unwrap();
    Ok(response)
}

#[command]
async fn gpt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // send the prompt
    let owned_args = args.to_owned();
    openai::set_key(SETTINGS.clone().openai);
    let prompt = ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(owned_args.message().into()),
        name: Some(msg.author.name.clone()),
        function_call: None,
    };
    let mut messages = vec![prompt];
    let response = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .user(msg.author.id.0.to_string())
        .functions(FUNCTIONS.clone())
        .function_call("auto")
        .create()
        .await?;
    let message = &response.choices.first().unwrap().message;
    let message: String = match message.function_call.as_ref() {
        Some(function) => {
            let arguments: serde_json::Value = serde_json::from_str(function.arguments.as_ref())?;
            match function.name.as_ref() {
                "get_weather" => {
                    // Get the weather
                    let weather = get_weather_json(&arguments["location"].to_string(), 0).await;
                    messages.push(message.clone());
                    messages.push(ChatCompletionMessage {
                        role: ChatCompletionMessageRole::Function,
                        name: Some(function.name.clone()),
                        content: Some(
                            json!({
                                "weather": weather["current_condition"]
                            })
                            .to_string(),
                        ),
                        function_call: None,
                    });
                    function_gpt(msg.author.id.0.to_string(), &messages)
                        .await?
                        .trim()
                        .into()
                }
                _ => unreachable!(),
            }
        }
        None => message.content.as_ref().unwrap().trim().into(),
    };
    // format message
    let discord_message = MessageBuilder::new().push(message).build();
    msg.channel_id.say(&ctx, discord_message).await?;
    Ok(())
}
