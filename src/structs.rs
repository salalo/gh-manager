use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    url: String,
    pub title: String,
    pub events_url: String,
    pub events: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub actor: Actor,
    pub event: String,
    created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct Actor {
    pub login: String,
}
