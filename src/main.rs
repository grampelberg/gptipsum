//! Generate lorem-upsum like text via a prompt using the Together API.
use std::io::Read;

use clap::Parser;
use clio::Input;
use eyre::Result;
use itertools::Itertools;
use rand::seq::SliceRandom;
use serde::Deserialize;

static KEY: Option<&str> = option_env!("API_KEY");

static ENDPOINT: &str = "https://api.together.xyz/v1/chat/completions";

#[derive(Deserialize, Debug)]
struct Message {
    content: String,
}
#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize, Debug)]
struct Output {
    choices: Vec<Choice>,
}

#[derive(Parser)]
struct Args {
    prompt: Input,
    emojis: Input,

    #[arg(long, env = "API_KEY", default_value = KEY.unwrap_or_default())]
    api_key: String,

    #[arg(long, default_value = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo")]
    model: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = Args::parse();

    let mut prompt = String::new();
    args.prompt.read_to_string(&mut prompt)?;

    let emojis = {
        let mut buf = String::new();
        args.emojis.read_to_string(&mut buf)?;

        buf
    };

    loop {
        let rng = &mut rand::thread_rng();

        let choice = emojis
            .lines()
            .collect::<Vec<&str>>()
            .choose_multiple(rng, 20)
            .join(" ");

        let result: Output = reqwest::Client::new()
            .post(ENDPOINT)
            .header("Authorization", format!("Bearer {}", args.api_key))
            .json(&serde_json::json!({
                "model": args.model,
                "max_tokens": 8192,
                "messages": [
                {
                    "role": "user",
                    "content": format!("these are my emojis: {choice}"),
                },
                {
                    "role": "user",
                    "content": format!("now is {}", chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
                },
                {
                    "role": "user",
                    "content": prompt,
                },
                {
                    "role": "user",
                    "content": "only return the log lines themselves.",
                }],
            }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        for l in result
            .choices
            .iter()
            .flat_map(|c| c.message.content.lines())
            .collect::<Vec<&str>>()
        {
            println!("{l}");

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }
}
