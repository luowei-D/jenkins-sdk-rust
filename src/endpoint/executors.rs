use super::Endpoint;

/// Endpoint for retrieving executor statistics from Jenkins.
pub struct ExecutorsInfoEndpoint;

impl Endpoint for ExecutorsInfoEndpoint {
    /// HTTP method used (GET).
    fn method(&self) -> &str {
        "GET"
    }

    /// API path for retrieving executor information.
    fn endpoint(&self) -> String {
        "computer/api/json".into()
    }
}
