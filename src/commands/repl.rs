use lazy_static::lazy_static;
use log::debug;
use maplit::hashmap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use std::collections::HashMap;

lazy_static! {
    static ref CODE: Regex =
        Regex::new(r".+\n+\x60{3}(\w+)\n([\s\S]*?)\x60{3}").expect("Compile regex");

    /// <discord_code_name, (repl_code_name, repl_file_ext)>
    static ref LANGUAGES: HashMap<&'static str, (&'static str, &'static str)> = hashmap!{
        "bash" => ("bash", ""),
        "c" => ("c", ".c"),
        "csharp" => ("csharp", ".cs"),
        "cpp" => ("cpp", ".cpp"),
        "clojure" => ("clojure", ".clj"),
        "cobol" => ("cobol", ".cob"),
        "coffeescript" => ("coffeescript", ".coffee"),
        "d" => ("d", ".d"),
        "elixir" => ("elixir", ".exs"),
        "erlang" => ("erlang", ".erl"),
        "fsharp" => ("fsharp", ".fs"),
        "go" => ("go", ".go"),
        "haskell" => ("haskell", ".hs"),
        "java" => ("java", ".java"),
        "javascript" => ("javascript", ".js"),
        "kotlin" => ("kotlin", ".kt"),
        "mysql" => ("mysql", ".sql"),
        "objc" => ("objective-c", ".m"),
        "perl" => ("perl", ".pl"),
        "php" => ("php", ".php"),
        "python" => ("python3", ".py"),
        "python2" => ("python2", ".py"),
        "r" => ("r", ".R"),
        "ruby" => ("ruby", ".rb"),
        "rust" => ("rust", ".rs"),
        "scala" => ("scala", ".scala"),
        "scheme" => ("scheme", ".scm"),
        "swift" => ("swift", ".swift"),
        "vb" => ("vb", ".vb"),
    };
}

#[derive(Debug, Serialize)]
pub struct Request {
    project: Project,
    run: bool,
    save: bool,
}

#[derive(Debug, Serialize)]
pub struct Project {
    language: String,
    network: bool,
    output_type: Option<String>,
    share: String,
    source_files: Vec<File>,
}

#[derive(Debug, Serialize)]
pub struct File {
    filename: String,
    body: String,
    position: u32,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    stdout: Option<String>,
    stderr: Option<String>,
    time: Option<String>,
    result: Option<RunnerResult>,
    build_stderr: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunnerResult {
    Success,
    Failure,
    Timeout,
}

#[command]
async fn repl(ctx: &Context, msg: &Message) -> CommandResult {
    let caps = if let Some(caps) = CODE.captures(&msg.content) {
        caps
    } else {
        msg.reply(
            &ctx,
            "Couldn't parse your code. Make sure you wrap it in language codeblocks.",
        )
        .await?;
        return Ok(());
    };

    // supported language?
    let (language_name, language_ext) = if let Some(language) = LANGUAGES.get(&caps[1]) {
        *language
    } else {
        msg.reply(&ctx, "The REPL doesn't support that language.")
            .await?;
        return Ok(());
    };

    // build request payload
    let payload = Request {
        project: Project {
            language: language_name.to_string(),
            network: true,
            output_type: None,
            share: "private".into(),
            source_files: vec![File {
                filename: format!("main{}", language_ext),
                body: caps[2].to_string(),
                position: 0,
            }],
        },
        run: true,
        save: false,
    };

    // make request to the playground
    let client = reqwest::blocking::Client::new();
    let res = match client
        .post("https://paiza.io/api/projects.json")
        .json(&payload)
        .send()
    {
        Ok(res) => res,
        Err(e) => {
            debug!("Error: {:#?}", e);
            msg.reply(&ctx, "There was an issue sending the code to the REPL.")
                .await?;
            return Ok(());
        }
    };

    // deserialize json response into struct
    let json: Response = match res.json() {
        Ok(json) => json,
        Err(e) => {
            debug!("Error: {:#?}", e);
            msg.reply(&ctx, "There was an issue with the response from the REPL.")
                .await?;
            return Ok(());
        }
    };

    // reply to user
    let message_builder = match json.result {
        Some(RunnerResult::Success) => MessageBuilder::new()
            .mention(&msg.author)
            .push(" ")
            .push("here's the output:")
            .push_codeblock(json.stdout.unwrap(), Some(&caps[1]))
            .push(format!("Code ran in: {}s", json.time.unwrap()))
            .build(),
        Some(RunnerResult::Failure) => MessageBuilder::new()
            .mention(&msg.author)
            .push(" ")
            .push("your compilation failed")
            .push_codeblock(json.stderr.unwrap(), Some(&caps[1]))
            .build(),
        Some(RunnerResult::Timeout) => MessageBuilder::new()
            .mention(&msg.author)
            .push(" your code timed out")
            .build(),
        None => {
            if let Some(stderr) = json.build_stderr {
                MessageBuilder::new()
                    .mention(&msg.author)
                    .push(" ")
                    .push("your compilation failed")
                    .push_codeblock(stderr, Some(&caps[1]))
                    .build()
            } else {
                MessageBuilder::new()
                    .mention(&msg.author)
                    .push(" ")
                    .push("your compilation failed")
                    .build()
            }
        }
    };
    if message_builder.len() >= 2000 {
        msg.reply(&ctx, "the response was too large...").await?;
    } else {
        msg.channel_id.say(&ctx, &message_builder).await?;
    }
    Ok(())
}
