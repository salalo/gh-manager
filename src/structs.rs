use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    url: String,
    pub title: String,
    pub events_url: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueWithEvents {
    pub title: String,
    pub events: Vec<Event>,
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
