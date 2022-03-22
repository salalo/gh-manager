use reqwest::header;
use reqwest::header::HeaderMap;
use std::env;

pub fn setup_headers() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    let token_var: String = env::var("TOKEN").unwrap_or("no token".into());
    let token: String = format!("token {}", &token_var);
    headers.insert(header::USER_AGENT, "request".parse().unwrap());
    headers.insert(header::AUTHORIZATION, token.parse().unwrap());

    headers
}
