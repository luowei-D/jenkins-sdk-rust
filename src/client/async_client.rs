use crate::{client::AsyncClient, error::JenkinsError};
use reqwest::Client;

/// Asynchronous Jenkins API client.
pub struct JenkinsAsyncClient {
    url: String,
    username: String,
    api_token: String,
    client: Client,
}

impl JenkinsAsyncClient {
    /// Creates a new asynchronous Jenkins API client.
    ///
    /// # Arguments
    ///
    /// * `url` - Base URL of the Jenkins server.
    /// * `username` - Username for authentication.
    /// * `api_token` - API token for authentication.
    pub fn new(url: &str, username: &str, api_token: &str) -> Self {
        Self {
            url: url.into(),
            username: username.into(),
            api_token: api_token.into(),
            client: Client::builder()
                .danger_accept_invalid_certs(true)
                .no_proxy()
                .build()
                .unwrap(),
        }
    }
}

#[async_trait::async_trait]
impl AsyncClient for JenkinsAsyncClient {
    /// Sends an asynchronous HTTP request to the Jenkins server.
    async fn request(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<String, JenkinsError> {
        let url = format!("{}/{}", self.url, endpoint);
        let req = self
            .client
            .request(method.parse()?, url)
            .basic_auth(&self.username, Some(&self.api_token))
            .header("User-Agent", "jenkins-sdk-rust");

        let resp = if let Some(p) = params {
            req.form(&p).send().await?
        } else {
            req.send().await?
        };

        Ok(resp.text().await?)
    }
}
