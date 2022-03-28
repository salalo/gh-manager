use crate::helpers::setup_headers;
use crate::structs::{Event, Issue};
use std::env;
use std::string::String;

// User interacted with an issue if: (below being actual gh api event names)
// added_to_project
// closed
// reopened
// commented
// commited

pub fn get_touched_issues(event: &Event) -> bool {
    let gh_events: Vec<&str> = vec![
        "added_to_project",
        "closed",
        "reopened",
        "commented",
        "commited",
        //"labeled",
    ];

    // check time of event creation? - depends on the gh api
    if event.actor.login == "salalo" && gh_events.contains(&&event.event[..]) {
        true
    } else {
        false
    }
}

pub async fn get_issue_events(events_url: &str) -> Vec<Event> {
    reqwest::Client::new()
        .get(events_url)
        .headers(setup_headers())
        .send()
        .await
        .unwrap()
        .json::<Vec<Event>>()
        .await
        .unwrap()
}

pub async fn get_issues() -> Vec<Issue> {
    let org_var: String = env::var("ORG").unwrap_or("no org".into());
    let repo_var: String = env::var("REPO").unwrap_or("no repo".into());

    let url: String = format!(
        "https://api.github.com/repos/{}/{}/issues",
        &org_var, &repo_var
    );

    reqwest::Client::new()
        .get(url)
        .headers(setup_headers())
        .send()
        .await
        .unwrap()
        .json::<Vec<Issue>>()
        .await
        .unwrap()
}
