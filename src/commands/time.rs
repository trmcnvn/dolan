use reqwest;
use select::document::Document;
use select::predicate::Attr;
use serenity::framework::standard::{Args, Command, CommandError as Error, CommandOptions};
use serenity::model::channel::Message;
use serenity::prelude::Context;
use std::sync::Arc;

pub struct Time;
impl Command for Time {
    fn options(&self) -> Arc<CommandOptions> {
        Arc::new(CommandOptions {
            desc: Some("Gets the current time in the timezone specified".into()),
            usage: Some("?time <timezone/city/country>".into()),
            min_args: Some(1),
            max_args: Some(1),
            ..CommandOptions::default()
        })
    }

    fn execute(&self, _context: &mut Context, message: &Message, args: Args) -> Result<(), Error> {
        let timezone = args.current().unwrap_or_default();
        let endpoint = format!("https://time.is/{}", timezone);
        let response = reqwest::get(&endpoint).unwrap();
        let document = Document::from_read(response).unwrap();
        let time = document.find(Attr("id", "twd")).next().unwrap().text();
        let human_timezone = document.find(Attr("id", "msgdiv")).next().unwrap().text();
        message.reply(&format!("{}: {}", human_timezone.trim(), time.trim()))?;
        Ok(())
    }
}
