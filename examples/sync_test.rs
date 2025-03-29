use jenkins_sdk::*;
use serde_json::json;

fn main() -> Result<(), JenkinsError> {
    // Replace these with your Jenkins URL, username, and API token.
    let client = JenkinsSyncClient::new("http://your-jenkins-url", "username", "api_token");

    let queue: serde_json::Value = Query::query(&QueueLength, &client)?;
    println!(
        "Queue Length: {}",
        queue["items"].as_array().map_or(0, |items| items.len())
    );

    let mut executors: ExecutorsInfo = Query::query(&ExecutorsInfoEndpoint, &client)?;
    executors.calculate_idle();
    println!("Executors Info: {:?}", executors);

    let jobs: serde_json::Value = Query::query(&JobsInfo, &client)?;
    println!("Jobs Info: {:?}", jobs);

    let console_text: String = Query::query(&ConsoleText("example-job", "1"), &client)?;
    println!("Console Output: {}", console_text);

    let build_params = json!({"param1": "value1", "param2": "value2"});

    RawQuery::raw_query(
        &TriggerBuild {
            job_name: "example-job",
            params: &build_params,
        },
        &client,
    )?;

    println!("Build triggered successfully.");

    // Specify the Jenkins job name and the build number you want to stop.
    let job_name = "example-job";
    let build_number = "123";

    // Perform the stop build request.
    RawQuery::raw_query(&StopBuild(job_name, build_number), &client)?;

    println!(
        "Build #{} of job '{}' stopped successfully.",
        build_number, job_name
    );

    Ok(())
}
