use crate::{client::SyncClient, error::JenkinsError};
use reqwest::blocking::Client;

/// Synchronous Jenkins API client.
pub struct JenkinsSyncClient {
    url: String,
    username: String,
    api_token: String,
    client: Client,
}

impl JenkinsSyncClient {
    /// Creates a new synchronous Jenkins API client.
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

impl SyncClient for JenkinsSyncClient {
    /// Sends a synchronous HTTP request to the Jenkins server.
    fn request(
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
            req.form(&p).send()?
        } else {
            req.send()?
        };

        Ok(resp.text()?)
    }
}
