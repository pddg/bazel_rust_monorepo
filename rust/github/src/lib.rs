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

fn interpret_error(status: u16) -> Error {
    assert!(status >= 400, "given status is not an error");
    if status < 500 {
        return Error::InvalidRequest(status);
    }
    return Error::Unavailable(status);
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
            return Err(interpret_error(status));
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn status_4xx_cause_invalid_request() {
        let actual = interpret_error(400);
        match actual {
            Error::InvalidRequest(_) => {}
            _ => {
                panic!("unexpected errror: {:?}", actual)
            }
        }
    }
    #[test]
    fn status_5xx_cause_unavailable_error() {
        let actual = interpret_error(500);
        match actual {
            Error::Unavailable(_) => {}
            _ => {
                panic!("unexpected errror: {:?}", actual)
            }
        }
    }
    #[test]
    #[should_panic]
    fn status_2xx_cause_panic() {
        interpret_error(200);
    }
}
