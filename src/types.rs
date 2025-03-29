use serde::{Deserialize, Serialize};

/// Represents a Jenkins job.
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    /// Name of the job.
    pub name: String,
    /// URL to access the job.
    pub url: String,
    /// Status color of the job (e.g., blue, red, yellow).
    pub color: String,
}

/// Information about Jenkins executors.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutorsInfo {
    /// Total number of executors available.
    pub total_executors: u32,
    /// Number of currently busy executors.
    pub busy_executors: u32,
    /// Calculated number of idle executors (not directly provided by Jenkins).
    #[serde(skip)]
    pub idle_executors: u32,
}

impl ExecutorsInfo {
    /// Calculates the number of idle executors.
    pub fn calculate_idle(&mut self) {
        self.idle_executors = self.total_executors - self.busy_executors;
    }
}
