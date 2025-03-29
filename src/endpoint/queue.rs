use super::Endpoint;

/// Endpoint for retrieving the length and details of the Jenkins build queue.
pub struct QueueLength;

impl Endpoint for QueueLength {
    /// HTTP method used (GET).
    fn method(&self) -> &str {
        "GET"
    }

    /// API path to retrieve queue information.
    fn endpoint(&self) -> String {
        "queue/api/json".into()
    }
}
