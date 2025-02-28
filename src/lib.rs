use reqwest::{Client, Method, Response, header::USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

/// Represents a Jenkins job.
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    /// The name of the job.
    pub name: String,
    /// The URL of the job.
    pub url: String,
    /// The status color of the job (e.g., blue, red, yellow).
    pub color: String,
}

/// Information about Jenkins executors.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutorsInfo {
    /// Total number of executors.
    pub total_executors: u32,
    /// Number of executors currently busy.
    pub busy_executors: u32,
    /// Number of executors currently idle.
    pub idle_executors: u32,
}

/// Jenkins client wrapper for interacting with the Jenkins API.
pub struct JenkinsClient {
    url: String,
    username: String,
    api_token: String,
    client: Client,
}

impl JenkinsClient {
    /// Creates a new instance of JenkinsClient.
    ///
    /// # Arguments
    ///
    /// * `url` - Base URL of the Jenkins server.
    /// * `username` - Username for authentication.
    /// * `api_token` - API token or password for authentication.
    pub fn new(url: &str, username: &str, api_token: &str) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .no_proxy()
            .build()
            .expect("Failed to build HTTP client");

        JenkinsClient {
            url: url.to_owned(),
            username: username.to_owned(),
            api_token: api_token.to_owned(),
            client,
        }
    }

    /// Sends an HTTP request to a specific Jenkins API endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint relative to the Jenkins base URL.
    /// * `method` - HTTP method (GET, POST, etc.).
    /// * `body` - Optional payload as a string.
    ///
    /// # Returns
    ///
    /// A reqwest `Response` object.
    async fn send_request(
        &self,
        endpoint: &str,
        method: Method,
        body: Option<String>,
    ) -> Result<Response, Box<dyn Error>> {
        let url = format!("{}/{}", self.url, endpoint);
        let mut request = self
            .client
            .request(method, &url)
            .basic_auth(&self.username, Some(&self.api_token))
            .header(
                USER_AGENT,
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
            );

        if let Some(payload) = body {
            request = request.body(payload);
        }

        Ok(request.send().await?)
    }

    /// Retrieves the current length of the Jenkins build queue.
    ///
    /// # Returns
    ///
    /// The number of jobs waiting in the queue.
    pub async fn get_queue_length(&self) -> Result<usize, Box<dyn Error>> {
        let response = self
            .send_request("queue/api/json", Method::GET, None)
            .await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(format!("Request failed: status {}, response {}", status, text).into());
        }

        let json: Value = serde_json::from_str(&text)?;
        Ok(json["items"].as_array().map_or(0, |items| items.len()))
    }

    /// Retrieves Jenkins executor statistics.
    ///
    /// # Returns
    ///
    /// An `ExecutorsInfo` object containing executor details.
    pub async fn get_executors_info(&self) -> Result<ExecutorsInfo, Box<dyn Error>> {
        let response = self
            .send_request("computer/api/json", Method::GET, None)
            .await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(format!("Request failed: status {}, response {}", status, text).into());
        }

        let json: Value = serde_json::from_str(&text)?;
        let total_executors = json["totalExecutors"].as_u64().unwrap_or(0) as u32;
        let busy_executors = json["busyExecutors"].as_u64().unwrap_or(0) as u32;

        Ok(ExecutorsInfo {
            total_executors,
            busy_executors,
            idle_executors: total_executors - busy_executors,
        })
    }

    /// Retrieves information on all Jenkins jobs.
    ///
    /// # Returns
    ///
    /// A vector containing details of each job.
    pub async fn get_jobs_info(&self) -> Result<Vec<Job>, Box<dyn Error>> {
        let response = self
            .send_request("api/json?tree=jobs[name,url,color]", Method::GET, None)
            .await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(format!("Request failed: status {}, response {}", status, text).into());
        }

        let json: Value = serde_json::from_str(&text)?;
        let jobs: Vec<Job> = serde_json::from_value(json["jobs"].clone())?;
        Ok(jobs)
    }

    /// Retrieves console output text for a specific build.
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the Jenkins job.
    /// * `build_number` - Specific build number.
    ///
    /// # Returns
    ///
    /// Console output as a plain string.
    pub async fn get_console_text(
        &self,
        job_name: &str,
        build_number: &str,
    ) -> Result<String, Box<dyn Error>> {
        let endpoint = format!("job/{}/{}/consoleText", job_name, build_number);
        let response = self.send_request(&endpoint, Method::GET, None).await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(format!("Request failed: status {}, response {}", status, text).into());
        }

        Ok(text)
    }

    /// Stops an ongoing build.
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the Jenkins job.
    /// * `build_number` - Build number to stop.
    pub async fn stop_build(
        &self,
        job_name: &str,
        build_number: &str,
    ) -> Result<(), Box<dyn Error>> {
        let endpoint = format!("job/{}/{}/stop", job_name, build_number);
        let response = self.send_request(&endpoint, Method::POST, None).await?;
        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            return Err(format!("Request failed: status {}, response {}", status, text).into());
        }

        Ok(())
    }

    /// Triggers a Jenkins build with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `job_name` - Name of the Jenkins job.
    /// * `params` - JSON parameters for the build.
    pub async fn trigger_build_with_params(
        &self,
        job_name: &str,
        params: &Value,
    ) -> Result<(), Box<dyn Error>> {
        let endpoint = format!("job/{}/buildWithParameters", job_name);
        let response = self
            .send_request(&endpoint, Method::POST, Some(params.to_string()))
            .await?;
        let status = response.status();

        if !status.is_success() {
            let text = response.text().await?;
            return Err(format!("Request failed: status {}, response {}", status, text).into());
        }

        Ok(())
    }
}
