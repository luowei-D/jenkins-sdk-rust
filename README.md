<div align=right>Table of Contents↗️</div>

<h1 align=center><code>jenkins-sdk-rust</code></h1>

<p align=center>📦 Jenkins API SDK written in Rust</p>

<div align=center>
  <a href="https://crates.io/crates/jenkins-sdk">
    <img src="https://img.shields.io/crates/v/jenkins-sdk.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/jenkins-sdk">
    <img src="https://img.shields.io/crates/dr/jenkins-sdk?color=ba86eb&logo=Handshake&logoColor=ea6aa6" alt="crates.io downloads">
  </a>
  <a href="https://github.com/lvillis/jenkins-sdk-rust">
    <img src="https://img.shields.io/github/repo-size/lvillis/jenkins-sdk-rust?style=flat-square&color=328657" alt="repo size">
  </a>
  <a href="https://github.com/lvillis/jenkins-sdk-rust/actions">
    <img src="https://github.com/lvillis/jenkins-sdk-rust/actions/workflows/ci.yml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%20jenkins-sdk-rust!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>
</div>

---

This project is a Jenkins API SDK written in Rust, designed to simplify interaction with Jenkins for Rust developers. The SDK supports asynchronous programming (via Tokio) and provides a unified interface to manage Jenkins jobs, queues, and executors efficiently.

## Features

- **Asynchronous Support**: Built on Tokio to support high concurrency.
- **Unified Request Handling**: Simplified API interface for consistent and easy interactions.
- **Detailed Documentation**: Clearly documented inputs and outputs for each method.
- **Comprehensive Error Handling**: Robust error handling with meaningful messages for ease of debugging.

## Implemented Interfaces

- **Job Management**
  - [x] Retrieve Jobs Information
  - [x] Retrieve Console Output
  - [x] Trigger Builds with Parameters
  - [x] Stop Builds

- **Queue Management**
  - [x] Retrieve Build Queue Length

- **Executor Management**
  - [x] Retrieve Executors Information

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
jenkins-sdk = "0.1"
```

## Quick Example

```rust
use jenkins_sdk::JenkinsClient;
use serde_json::json;

#[tokio::main]
async fn main() {
    let client = JenkinsClient::new("https://jenkins.example.com", "username", "api_token");

    match client.get_jobs_info().await {
        Ok(jobs) => println!("Jobs: {:?}", jobs),
        Err(e) => eprintln!("Error fetching jobs: {}", e),
    }

    // Trigger a build with parameters
    let params = json!({ "param1": "value1" });
    if let Err(e) = client.trigger_build_with_params("MyJob", &params).await {
        eprintln!("Failed to trigger build: {}", e);
    }
}
```

## License

This project is licensed under the MIT License.
