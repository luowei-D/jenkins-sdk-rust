/// Trait representing a Jenkins API endpoint.
pub trait Endpoint {
    /// Returns the HTTP method (GET, POST, etc.) for this endpoint.
    fn method(&self) -> &str;

    /// Returns the specific Jenkins API endpoint URL.
    fn endpoint(&self) -> String;

    /// Optional query or form parameters.
    fn params(&self) -> Option<Vec<(&str, &str)>> {
        None
    }
}

pub mod builds;
pub mod executors;
pub mod jobs;
pub mod queue;

pub use builds::*;
pub use executors::*;
pub use jobs::*;
pub use queue::*;
