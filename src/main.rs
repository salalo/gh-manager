use crate::structs::IssueWithEvents;
mod helpers;
mod issues;
mod structs;

#[tokio::main]
async fn main() {
    println!("Fetching todays issues you've worked on...");

    let mut filtered_issues = Vec::<IssueWithEvents>::new();
    let mut issues_titles = Vec::<String>::new();

    for issue in issues::get_issues().await {
        let events_issue = IssueWithEvents {
            events: issues::get_issue_events(&issue.events_url).await,
            issue,
        };

        let issue_valid = events_issue
            .events
            .iter()
            .any(|e| issues::get_touched_issues(&e));

        if issue_valid {
            issues_titles.push(events_issue.issue.title.clone());
            filtered_issues.push(events_issue);
        }
    }

    for title in issues_titles {
        println!("{:?}", title);
    }
}
