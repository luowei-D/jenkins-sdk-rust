use jenkins_sdk::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), JenkinsError> {
    // Replace these with your Jenkins URL, username, and API token.
    let client = JenkinsAsyncClient::new("http://your-jenkins-url", "username", "api_token");

    let queue: serde_json::Value = AsyncQuery::query(&QueueLength, &client).await?;
    println!(
        "Queue Length: {}",
        queue["items"].as_array().map_or(0, |items| items.len())
    );

    let mut executors: ExecutorsInfo = AsyncQuery::query(&ExecutorsInfoEndpoint, &client).await?;
    executors.calculate_idle();
    println!("Executors Info: {:?}", executors);

    let jobs: serde_json::Value = AsyncQuery::query(&JobsInfo, &client).await?;
    println!("Jobs Info: {:?}", jobs);

    let console_text: String =
        AsyncRawQuery::raw_query(&ConsoleText("example-job", "1"), &client).await?;
    println!("Console Output: {}", console_text);

    let build_params = json!({"param1": "value1", "param2": "value2"});

    AsyncRawQuery::raw_query(
        &TriggerBuild {
            job_name: "example-job",
            params: &build_params,
        },
        &client,
    )
    .await?;

    println!("Build triggered successfully.");

    // Specify the Jenkins job name and the build number you want to stop.
    let job_name = "example-job";
    let build_number = "123";

    // Perform the stop build request.
    AsyncRawQuery::raw_query(&StopBuild(job_name, build_number), &client).await?;

    println!(
        "Build #{} of job '{}' stopped successfully.",
        build_number, job_name
    );

    Ok(())
}
