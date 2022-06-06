mod navigate_value;

use crate::navigate_value::NavigateValue;
use anyhow::{bail, Context, Result};
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header;
use serde_json::{json, Value};

#[derive(Debug, Parser)]
#[clap(about, author)]
pub struct Config {
    /// A GitHub access token, created with the `repo` and `read:user` scopes.
    ///
    /// You can make one of these st https://github.com/settings/tokens
    #[clap(env = "GITHUB_API_TOKEN")]
    github_api_token: String,

    /// This caption will be printed by the count when you have no review requests
    #[clap(long = "done-caption", default_value = "✨")]
    done_caption: String,

    /// This caption will be printed by the count when you have outstanding
    /// review requests
    #[clap(long = "todo-caption", default_value = "👀")]
    todo_caption: String,
}

fn main() {
    env_logger::Builder::from_env("XBAR_REVIEW_REQUEST_STATUS_LOG").init();

    if let Err(err) = try_main() {
        println!("{:?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let config = Config::parse();

    let prs = fetch(&config.github_api_token).context("could not fetch review requests")?;

    let mut menu_lines: Vec<String> = Vec::new();

    for pr_value in prs.get_array("/data/search/nodes")? {
        menu_lines.push(format!(
            "{} by {} | href={}",
            pr_value.get_str("/title")?,
            pr_value.get_str("/author/login")?,
            pr_value.get_str("/url")?
        ))
    }

    if menu_lines.is_empty() {
        println!("0 {}", config.done_caption);
    } else {
        println!("{} {}", menu_lines.len(), config.todo_caption);
        println!("---");
        for line in menu_lines {
            println!("{line}");
        }
    }

    Ok(())
}

fn fetch(api_token: &str) -> Result<Value> {
    let client = Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .context("could not build the HTTP client")?;

    let response = client
        .post("https://api.github.com/graphql")
        .header(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", api_token))
                .context("could not create an Authorization header from the specified token")?,
        )
        .json(&json!({ "query": include_str!("review_requests.graphql") }))
        .send()
        .context("could not request data from GitHub's API")?;

    let body: Value = response.json().context("could not read JSON body")?;

    if let Some(value) = body.pointer("errors") {
        match value {
            Value::Null => (),
            Value::Array(errs) => {
                for err in errs {
                    log::error!("{}", err);
                }
            }
            _ => bail!("errors was not an array"),
        }
    }

    Ok(body)
}
