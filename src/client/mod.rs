use crate::error::JenkinsError;

#[async_trait::async_trait]
pub trait AsyncClient: Sync {
    async fn request(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<String, JenkinsError>;
}

pub trait SyncClient {
    fn request(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<String, JenkinsError>;
}

pub mod async_client;
pub mod sync_client;

pub use async_client::JenkinsAsyncClient;
pub use sync_client::JenkinsSyncClient;
