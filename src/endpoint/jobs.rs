use super::Endpoint;

/// Endpoint for retrieving information about Jenkins jobs.
pub struct JobsInfo;

impl Endpoint for JobsInfo {
    /// HTTP method used (GET).
    fn method(&self) -> &str {
        "GET"
    }

    /// API path for retrieving job information.
    fn endpoint(&self) -> String {
        "api/json?tree=jobs[name,url,color]".into()
    }
}
