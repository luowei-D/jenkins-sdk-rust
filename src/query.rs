use crate::{client::*, endpoint::Endpoint, error::JenkinsError};
use serde::de::DeserializeOwned;

/// Trait for asynchronously querying Jenkins endpoints that return JSON data.
#[async_trait::async_trait]
pub trait AsyncQuery<T> {
    /// Executes an asynchronous request and deserializes the JSON response.
    ///
    /// # Arguments
    ///
    /// * `client` - The asynchronous Jenkins client to perform the request.
    ///
    /// # Returns
    ///
    /// * `Result<T, JenkinsError>` - The deserialized response.
    async fn query(&self, client: &impl AsyncClient) -> Result<T, JenkinsError>;
}

/// Trait for synchronously querying Jenkins endpoints that return JSON data.
pub trait Query<T> {
    /// Executes a synchronous request and deserializes the JSON response.
    ///
    /// # Arguments
    ///
    /// * `client` - The synchronous Jenkins client to perform the request.
    ///
    /// # Returns
    ///
    /// * `Result<T, JenkinsError>` - The deserialized response.
    fn query(&self, client: &impl SyncClient) -> Result<T, JenkinsError>;
}

/// Implements synchronous query behavior for all Endpoints returning JSON.
impl<E, T> Query<T> for E
where
    E: Endpoint,
    T: DeserializeOwned,
{
    fn query(&self, client: &impl SyncClient) -> Result<T, JenkinsError> {
        let res = client.request(self.method(), &self.endpoint(), self.params().as_deref())?;
        Ok(serde_json::from_str(&res)?)
    }
}

/// Implements asynchronous query behavior for all Endpoints returning JSON.
#[async_trait::async_trait]
impl<E, T> AsyncQuery<T> for E
where
    E: Endpoint + Sync,
    T: DeserializeOwned + Send,
{
    async fn query(&self, client: &impl AsyncClient) -> Result<T, JenkinsError> {
        let res = client
            .request(self.method(), &self.endpoint(), self.params().as_deref())
            .await?;
        Ok(serde_json::from_str(&res)?)
    }
}

/// Trait for asynchronously querying Jenkins endpoints that return raw text.
#[async_trait::async_trait]
pub trait AsyncRawQuery {
    /// Executes an asynchronous request and returns the raw response text.
    ///
    /// # Arguments
    ///
    /// * `client` - The asynchronous Jenkins client to perform the request.
    ///
    /// # Returns
    ///
    /// * `Result<String, JenkinsError>` - The raw response text.
    async fn raw_query(&self, client: &impl AsyncClient) -> Result<String, JenkinsError>;
}

/// Trait for synchronously querying Jenkins endpoints that return raw text.
pub trait RawQuery {
    /// Executes a synchronous request and returns the raw response text.
    ///
    /// # Arguments
    ///
    /// * `client` - The synchronous Jenkins client to perform the request.
    ///
    /// # Returns
    ///
    /// * `Result<String, JenkinsError>` - The raw response text.
    fn raw_query(&self, client: &impl SyncClient) -> Result<String, JenkinsError>;
}

/// Implements asynchronous query behavior for endpoints returning raw text.
#[async_trait::async_trait]
impl<E> AsyncRawQuery for E
where
    E: Endpoint + Sync,
{
    async fn raw_query(&self, client: &impl AsyncClient) -> Result<String, JenkinsError> {
        client
            .request(self.method(), &self.endpoint(), self.params().as_deref())
            .await
    }
}

/// Implements synchronous query behavior for endpoints returning raw text.
impl<E> RawQuery for E
where
    E: Endpoint,
{
    fn raw_query(&self, client: &impl SyncClient) -> Result<String, JenkinsError> {
        client.request(self.method(), &self.endpoint(), self.params().as_deref())
    }
}
