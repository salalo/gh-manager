mod helpers;
mod issues;
mod structs;

#[tokio::main]
async fn main() {
    println!("Fetching todays issues you've worked on...");

    for mut issue in issues::get_issues().await {
        for event in issues::get_issue_events(&issue.events_url).await {
            if issues::get_touched_issues(&event) {
                issue.events = Some(true);
            }
        }

        if issue.events.is_some() {
            println!("`{}`", issue.title);
        }
    }
}
