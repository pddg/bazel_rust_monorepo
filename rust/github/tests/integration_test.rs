use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use github::{Client, Error, Release};

#[tokio::test]
async fn get_latest_release_success() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repos/test_owner/test_repo/releases/latest"))
        .respond_with(ResponseTemplate::new(200).set_body_string(
            r#"
                {
                    "url": "http://localhost:80/test",
                    "tag_name": "v1.0.0",
                    "name": "Release v1.0.0"
                }
            "#,
        ))
        .mount(&mock_server)
        .await;
    let client = Client::new(&mock_server.uri());
    let actual = client
        .get_latest_release("test_owner", "test_repo")
        .await
        .unwrap();
    let expected = Release {
        url: "http://localhost:80/test".to_string(),
        tag_name: "v1.0.0".to_string(),
        name: "Release v1.0.0".to_string(),
    };
    assert_eq!(actual, expected);
}

#[tokio::test]
async fn get_latest_release_invalid_request() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repos/test_owner/test_repo/releases/latest"))
        .respond_with(ResponseTemplate::new(400))
        .mount(&mock_server)
        .await;
    let client = Client::new(&mock_server.uri());
    let actual = client.get_latest_release("test_owner", "test_repo").await;
    match actual {
        Ok(_) => panic!("the request should not be succeeded"),
        Err(e) => match e {
            Error::InvalidRequest(_) => {
                // ok
            }
            _ => panic!("unexpected error: {}", e),
        },
    }
}

#[tokio::test]
async fn get_latest_release_unavailable() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/repos/test_owner/test_repo/releases/latest"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    let client = Client::new(&mock_server.uri());
    let actual = client.get_latest_release("test_owner", "test_repo").await;
    match actual {
        Ok(_) => panic!("the request should not be succeeded"),
        Err(e) => match e {
            Error::Unavailable(_) => {
                // ok
            }
            _ => panic!("unexpected error: {}", e),
        },
    }
}
