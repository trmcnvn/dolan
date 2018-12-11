use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use reqwest;
use serde_derive::{Deserialize, Serialize};
use serenity::command;
use serenity::utils::MessageBuilder;
use hashbrown::HashMap;

/// Taken from `maplit` crate to work with hashbrown.
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::hashbrown::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

lazy_static! {
    static ref CODE: Regex =
        Regex::new(r".+\n+\x60{3}(\w+)\n([\s\S]*?)\x60{3}").expect("Compile regex");

    /// <discord_code_name, (repl_code_name, repl_file_ext)>
    static ref LANGUAGES: HashMap<&'static str, (&'static str, &'static str)> = hashmap!{
        "bash" => ("bash", ""),
        "c" => ("c", ".c"),
        "csharp" => ("csharp", ".cs"),
        "cpp" => ("cpp", ".cpp"),
        "cobol" => ("cobol", ".cob"),
        "clojure" => ("clojure", ".clj"),
        "coffeescript" => ("coffeescript", ".coffee"),
        "d" => ("d", ".d"),
        "elixir" => ("elixir", ".exs"),
        "erlang" => ("erlang", ".erl"),
        "fsharp" => ("fsharp", ".fs"),
        "go" => ("go", ".go"),
        "haskell" => ("haskell", ".hs"),
        "java" => ("java", ".java"),
        "kotlin" => ("kotlin", ".kt"),
        "mysql" => ("mysql", ".sql"),
        "objc" => ("objective-c", ".m"),
        "perl" => ("perl", ".pl"),
        "python" => ("python", ".py"),
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
}

command!(command(_context, message) {
    let caps = if let Some(caps) = CODE.captures(&message.content) {
        caps
    } else {
        message.reply("Couldn't parse your code. Make sure you wrap it in language codeblocks.")?;
        return Ok(());
    };

    // supported language?
    let (language_name, language_ext) = if let Some(language) = LANGUAGES.get(&caps[1]) {
        language
    } else {
        message.reply("The REPL doesn't support that language.")?;
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
    debug!("REPL payload: {:?}", payload);

    // make request to the playground
    let client = reqwest::Client::new();
    let mut res = match client.post("https://paiza.io/api/projects.json").json(&payload).send() {
        Ok(res) => res,
        Err(e) => {
            debug!("Error: {:#?}", e);
            message.reply("There was an issue sending the code to the REPL.")?;
            return Ok(());
        }
    };

    // deserialize json response into struct
    let json: Response = match res.json() {
        Ok(json) => json,
        Err(e) => {
            debug!("Error: {:#?}", e);
            message.reply("There was an issue with the response from the REPL.")?;
            return Ok(());
        }
    };
    debug!("REPL response: {:?}", json);

    // reply to user
    let message_builder = match json.result {
        Some(RunnerResult::Success) => {
            MessageBuilder::new()
                .mention(&message.author)
                .push(" ")
                .push("here's the output:")
                .push_codeblock(json.stdout.unwrap(), Some(&caps[1]))
                .push(format!("Code ran in: {}s", json.time.unwrap()))
                .build()
        }
        Some(RunnerResult::Failure) => {
            MessageBuilder::new()
                .mention(&message.author)
                .push(" ")
                .push("your compilation failed... yikes...")
                .push_codeblock(json.stderr.unwrap(), Some(&caps[1]))
                .build()
        }
        None => {
            if let Some(stderr) = json.build_stderr {
                MessageBuilder::new()
                    .mention(&message.author)
                    .push(" ")
                    .push("your compilation failed... yikes...")
                    .push_codeblock(stderr, Some(&caps[1]))
                    .build()
            } else {
                MessageBuilder::new()
                    .mention(&message.author)
                    .push(" ")
                    .push("your compilation failed... yikes...")
                    .build()
            }
        }
    };
    message.channel_id.say(&message_builder)?;
});
