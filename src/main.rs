mod issues;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let issues = issues::get_issues().await;

    println!("{:#?}", issues);

    Ok(())
}
