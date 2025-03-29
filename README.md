<div align=right>Table of Contents↗️</div>

<h1 align=center><code>jenkins-sdk-rust</code></h1>

<p align=center>📦 Jenkins API SDK written in Rust</p>

<div align=center>
  <a href="https://crates.io/crates/jenkins-sdk">
    <img src="https://img.shields.io/crates/v/jenkins-sdk.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/jenkins-sdk">
    <img src="https://img.shields.io/crates/dr/jenkins-sdk?color=ba86eb" alt="crates.io downloads">
  </a>
  <a href="https://github.com/lvillis/jenkins-sdk-rust">
    <img src="https://img.shields.io/github/repo-size/lvillis/jenkins-sdk-rust?style=flat-square&color=328657" alt="repo size">
  </a>
  <a href="https://github.com/lvillis/jenkins-sdk-rust/actions">
    <img src="https://github.com/lvillis/jenkins-sdk-rust/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%20jenkins-sdk-rust!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>
</div>

---

**jenkins-sdk-rust** is an ergonomic Rust SDK designed for interacting seamlessly with Jenkins APIs. Inspired by popular Rust SDKs such as the GitLab SDK, it provides a structured, idiomatic Rust API with support for both synchronous and asynchronous requests.

## ✨ Features

- **Async & Sync Clients**: Support for both synchronous and asynchronous workflows using Tokio and Reqwest.
- **Idiomatic Rust Interface**: Designed with clear traits and endpoint structures following Rust best practices.
- **Flexible Response Handling**: Built-in support for JSON deserialization and raw text responses.
- **Robust Error Handling**: Comprehensive error management with clear, descriptive error messages.
- **Comprehensive Documentation**: Rich documentation compatible with `cargo doc`.

## 🚀 Supported API Endpoints

- **Job Management**
  - [x] Retrieve jobs information
  - [x] Fetch console logs
  - [x] Trigger builds with parameters
  - [x] Stop ongoing builds

- **Queue Management**
  - [x] Retrieve build queue details

- **Executor Management**
  - [x] Retrieve executor statistics and status

## 📥 Installation

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
jenkins-sdk = "0.1"
```

## ⚡Quick Start

### Async Example
```rust
use jenkins_sdk::{JenkinsAsyncClient, AsyncQuery, JobsInfo};
use tokio;

#[tokio::main]
async fn main() -> Result<(), jenkins_sdk::JenkinsError> {
  let client = JenkinsAsyncClient::new("https://jenkins.example.com", "username", "api_token");

  // Retrieve job information
  let jobs: serde_json::Value = AsyncQuery::query(&JobsInfo, &client).await?;
  println!("Jobs: {:?}", jobs);

  Ok(())
}

```

### Sync Example

```rust
use jenkins_sdk::{JenkinsSyncClient, Query, JobsInfo};

fn main() -> Result<(), jenkins_sdk::JenkinsError> {
    let client = JenkinsSyncClient::new("https://jenkins.example.com", "username", "api_token");

    // Retrieve job information
    let jobs: serde_json::Value = Query::query(&JobsInfo, &client)?;
    println!("Jobs: {:?}", jobs);

    Ok(())
}
```

## 📃 License

This project is licensed under the MIT License.
