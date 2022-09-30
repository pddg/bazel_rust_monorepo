use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Release {
    url: String,
    tag_name: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let resp = client
        .get("https://api.github.com/repos/bazelbuild/rules_rust/releases/latest")
        // User-Agentがないと403で蹴られる
        // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
        .header("User-Agent", "sample")
        .send()
        .await?;
    if resp.status() != 200 {
        return Err(anyhow!("invalid status code"));
    }
    let j = resp.json::<Release>().await?;
    println!("github.com/bazelbuild/rules_rust");
    println!("Latest release is {}", j.name);
    println!("And its tag is {}", j.tag_name);
    println!("{}", j.url);
    Ok(())
}
