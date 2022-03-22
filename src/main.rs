mod helpers;
mod issues;
mod structs;

#[tokio::main]
async fn main() {
    let issues = issues::get_issues().await;

    for issue in issues {
        let events = issues::get_issue_events(&issue.events_url).await;
        println!("{:?}", issue);
        println!("{:?}", events);
    }
}
