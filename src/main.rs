mod issues;

#[tokio::main]
async fn main() {
    let issues = issues::get_issues().await;

    for issue in issues {
        println!("{:?}", issue);
    }
}
