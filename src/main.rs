use crate::structs::IssueWithEvents;
mod helpers;
mod issues;
mod structs;

#[tokio::main]
async fn main() {
    let issues = issues::get_issues().await;

    for issue in issues {
        let new_issue = IssueWithEvents {
            events: issues::get_issue_events(&issue.events_url).await,
            issue,
        };

        println!("{:?}", new_issue);
    }
}
