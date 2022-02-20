use reqwest::header;
use serde::Deserialize;
use std::env;
use std::string::String;

#[derive(Deserialize, Debug)]
pub struct Assignee {
    login: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    url: String,
    title: String,
    updated_at: String,
    assignees: Vec<Assignee>,
}

pub async fn get_issues() -> Vec<Response> {
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

    reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json::<Vec<Response>>()
        .await
        .unwrap()
}
