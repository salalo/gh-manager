use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    url: String,
    title: String,
    pub events_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    actor: Actor,
    event: String,
}
#[derive(Deserialize, Debug)]
struct Actor {
    login: String,
}
