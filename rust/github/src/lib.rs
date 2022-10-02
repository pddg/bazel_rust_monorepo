use reqwest::Client as HttpClient;
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid request: status code = {0}")]
    InvalidRequest(u16),
    #[error("server is unavailable: status code = {0}")]
    Unavailable(u16),
    #[error("unexpected error")]
    Reqwest(#[from] reqwest::Error),
}

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Client {
        return Client {
            base_url: base_url.to_string(),
        };
    }

    pub async fn get_latest_release(&self, owner: &str, repo: &str) -> Result<Release, Error> {
        let client = HttpClient::new();
        let resp = client
            .get(format!(
                "{}/repos/{}/{}/releases/latest",
                self.base_url, owner, repo
            ))
            // User-Agentがないと403で蹴られる
            // https://docs.github.com/en/rest/overview/resources-in-the-rest-api#user-agent-required
            .header("User-Agent", "sample")
            .send()
            .await?;
        let status = resp.status().as_u16();
        if status != 200 {
            if status < 500 {
                return Err(Error::InvalidRequest(status));
            }
            return Err(Error::Unavailable(status));
        }
        Ok(resp.json::<Release>().await?)
    }
}

#[derive(Deserialize)]
pub struct Release {
    pub url: String,
    pub tag_name: String,
    pub name: String,
}
