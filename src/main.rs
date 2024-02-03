use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// Request structure expecting two strings.
#[derive(Deserialize)]
struct Request {
    command1: String,
    command2: String,
}

/// Response structure returning a concatenated string.
#[derive(Serialize)]
struct Response {
    msg: String,
}

/// Handler function to concatenate command1 and command2 from the request.
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract commands from the request
    let Request { command1, command2 } = event.payload;

    // Concatenate command1 and command2
    let concatenated_commands = format!("{} {}", command1, command2);

    // Prepare the response
    let resp = Response {
        msg: concatenated_commands,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_target(false) // Disable printing the name of the module in every log line
        .without_time() // Disable time since CloudWatch adds ingestion time
        .init();

    // Run the Lambda function handler
    run(service_fn(function_handler)).await
}
