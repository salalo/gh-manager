use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    url: String,
    title: String,
    pub events_url: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueWithEvents {
    pub issue: Issue,
    pub events: Vec<Event>,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub actor: Actor,
    event: String,
    created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct Actor {
    pub login: String,
}
