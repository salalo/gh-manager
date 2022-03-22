use crate::structs::IssueWithEvents;
mod helpers;
mod issues;
mod structs;

#[tokio::main]
async fn main() {
    for issue in issues::get_issues().await {
        let new_issue = IssueWithEvents {
            events: issues::get_issue_events(&issue.events_url).await,
            issue,
        };

        println!("{:?}", new_issue);
    }
}
