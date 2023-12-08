use log::debug;
use reqwest::{self, header::USER_AGENT};
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use urlencoding::encode;

pub async fn get_weather_json(location: &str, weather_type: u32) -> serde_json::Value {
    let client = reqwest::Client::new();
    let endpoint = format!(
        "https://wttr.in/{}?m{}FqT&lang=en&format=j1",
        location, weather_type
    );
    let request = client.get(&endpoint).header(USER_AGENT, "curl");
    let response = request.send().await.unwrap();
    response.json().await.unwrap()
}

pub async fn get_weather_pretty(location: &str, weather_type: u32) -> String {
    let client = reqwest::Client::new();
    let endpoint = format!("https://wttr.in/{}?m{}FqT&lang=en", location, weather_type);
    let request = client.get(&endpoint).header(USER_AGENT, "curl");
    let response = request.send().await.unwrap();
    response.text().await.unwrap()
}

#[command]
async fn weather(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut owned_args = args.to_owned();
    let weather_type = owned_args.single::<u32>().unwrap_or(0);
    let locations: Vec<&str> = owned_args.rest().split(';').collect();
    let mut messages = Vec::with_capacity(locations.len());

    for location in locations {
        let valid_location = encode(location);
        if valid_location.trim().is_empty() {
            continue;
        }
        let message = get_weather_pretty(&valid_location, weather_type).await;
        messages.push((valid_location, message));
    }

    for (location, message) in messages {
        if message.len() >= 2000 {
            debug!("Message was too long, converting to image...");
            let embed = CreateEmbed::new().image(format!(
                "https://wttr.in{}_m{}Fq_lang=en.png",
                location, weather_type
            ));
            let builder = CreateMessage::new().embed(embed);
            msg.channel_id.send_message(&ctx, builder).await?;
        } else {
            let message_builder = MessageBuilder::new().push_codeblock(message, None).build();
            msg.channel_id.say(&ctx, &message_builder).await?;
        }
    }

    Ok(())
}
