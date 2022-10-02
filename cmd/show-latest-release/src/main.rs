use anyhow::Result;
use github::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("https://api.github.com");
    let latest_release = client
        .get_latest_release("bazelbuild", "rules_rust")
        .await?;
    println!("github.com/bazelbuild/rules_rust");
    println!("Latest release is {}", latest_release.name);
    println!("And its tag is {}", latest_release.tag_name);
    println!("{}", latest_release.url);
    Ok(())
}
