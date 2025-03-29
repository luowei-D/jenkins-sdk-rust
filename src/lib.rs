//! # jenkins-sdk
//!
//! An ergonomic, idiomatic Rust SDK for interacting with Jenkins APIs.
//!
//! This crate provides both synchronous and asynchronous interfaces,
//! structured endpoints, and robust error handling, simplifying Jenkins
//! API interactions in Rust applications.
//!
//! ## Features
//!
//! - **Asynchronous and Synchronous support** powered by Tokio and Reqwest.
//! - **Structured endpoint definitions** for clear and type-safe API calls.
//! - **Comprehensive error handling** with descriptive, easy-to-debug messages.
//! - **Flexible response handling** supporting both JSON and raw text.
//!
//! ## Quick Start
//!
//! ### Async Example
//!
//! ```rust
//! use jenkins_sdk::{JenkinsAsyncClient, AsyncQuery, JobsInfo};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), jenkins_sdk::JenkinsError> {
//!     let client = JenkinsAsyncClient::new(
//!         "https://jenkins.example.com",
//!         "username",
//!         "api_token"
//!     );
//!
//!     let jobs: serde_json::Value = AsyncQuery::query(&JobsInfo, &client).await?;
//!     println!("Jobs: {:?}", jobs);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Sync Example
//!
//! ```rust
//! use jenkins_sdk::{JenkinsSyncClient, Query, JobsInfo};
//!
//! fn main() -> Result<(), jenkins_sdk::JenkinsError> {
//!     let client = JenkinsSyncClient::new(
//!         "https://jenkins.example.com",
//!         "username",
//!         "api_token"
//!     );
//!
//!     let jobs: serde_json::Value = Query::query(&JobsInfo, &client)?;
//!     println!("Jobs: {:?}", jobs);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! Licensed under MIT License.

pub mod client;
pub mod endpoint;
pub mod error;
pub mod query;
pub mod types;

pub use client::{JenkinsAsyncClient, JenkinsSyncClient};
pub use endpoint::*;
pub use error::*;
pub use query::*;
pub use types::*;
