use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;
use std::string::String;

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Assignee {
    Simple(String),
    Assignee { login: String },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Simple(String),
    Assignee {
        login: String,
    },
    Issue {
        url: String,
        title: String,
        updated_at: String,
        assignees: Vec<Assignee>,
    },
}

pub async fn get_issues() -> Result<Vec<Response>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();

    let token_var: String = env::var("TOKEN").unwrap_or("no token".into());
    let org_var: String = env::var("ORG").unwrap_or("no org".into());
    let repo_var: String = env::var("REPO").unwrap_or("no repo".into());

    let url: String = format!(
        "https://api.github.com/repos/{}/{}/issues",
        &org_var, &repo_var
    );
    let token: String = format!("token {}", &token_var);

    headers.insert(header::USER_AGENT, "request".parse().unwrap());
    headers.insert(header::AUTHORIZATION, token.parse().unwrap());

    let res = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json::<Vec<Response>>()
        .await?;

    Ok(res)
}
