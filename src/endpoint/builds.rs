use super::Endpoint;
use serde_json::Value;

/// Endpoint to retrieve the console output text of a specific job build.
pub struct ConsoleText<'a>(pub &'a str, pub &'a str);

impl<'a> Endpoint for ConsoleText<'a> {
    /// HTTP method used (GET).
    fn method(&self) -> &str {
        "GET"
    }

    /// API path for retrieving console output for a specific build.
    fn endpoint(&self) -> String {
        format!("job/{}/{}/consoleText", self.0, self.1)
    }
}

/// Endpoint to stop a specific running build.
pub struct StopBuild<'a>(pub &'a str, pub &'a str);

impl<'a> Endpoint for StopBuild<'a> {
    /// HTTP method used (POST).
    fn method(&self) -> &str {
        "POST"
    }

    /// API path to stop a specific build.
    fn endpoint(&self) -> String {
        format!("job/{}/{}/stop", self.0, self.1)
    }
}

/// Endpoint to trigger a Jenkins job build with specific parameters.
pub struct TriggerBuild<'a> {
    /// Name of the Jenkins job.
    pub job_name: &'a str,
    /// Parameters required for the Jenkins job build (JSON).
    pub params: &'a Value,
}

impl<'a> Endpoint for TriggerBuild<'a> {
    /// HTTP method used (POST).
    fn method(&self) -> &str {
        "POST"
    }

    /// API path to trigger a Jenkins build with parameters.
    fn endpoint(&self) -> String {
        format!("job/{}/buildWithParameters", self.job_name)
    }

    /// Parameters to send with the request.
    fn params(&self) -> Option<Vec<(&str, &str)>> {
        self.params.as_object().map(|obj| {
            obj.iter()
                .map(|(k, v)| (k.as_str(), v.as_str().unwrap_or("")))
                .collect()
        })
    }
}
