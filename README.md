[![pipeline status](https://gitlab.com/ly178/tinayiluo_ids721_week2/badges/main/pipeline.svg)](https://gitlab.com/ly178/tinayiluo_ids721_week2/-/commits/main)

# Serverless AWS Lambda Function

## Goal
This project leveraged the power of serverless architecture to streamline data processing workflows. The solution employs an AWS Lambda function, meticulously crafted in the Rust programming language, utilizing Cargo Lambda for seamless deployment and management. 

## Overview 
This project is designed to offer a robust and scalable approach to handling data transformations by concatinating two strings together into one string, making it ideal for a wide range of applications.

## Setup

### Rust Lambda 
- Install Rust and Cargo Lambda: `brew install rust`, `brew tap cargo-lambda/cargo-lambda`, and `brew install cargo-lambda`.
- Check cargo lambda is successfully installed : `cargo lambda --version`.
- Create a new repo `new-lambda-project` on gitlab/github and clone it with HTTPS/SSH to local : `git clone <HTTPS/SSH>`.
- Go to one directory above the directory that `new-lambda-project` is located, set up Cargo Lambda project : `cargo lambda new new-lambda-project`.

### Data Processing
At the heart of the project is the data processing capability. Written in Rust, the function concatinate two strings together into one string.

- Add main function in `src/main.rs`
```
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

```
- Add dependencies in `Cargo.toml`
```
[dependencies]

lambda_runtime = "0.8.3"
serde = "1.0.136"
tokio = { version = "1", features = ["macros"] }
tracing = { version = "0.1", features = ["log"] }
tracing-subscriber = { version = "0.3", default-features = false, features = ["env-filter", "fmt"] }
```
- Do `cargo lambda watch` to test locally.
- Do `cargo lambda invoke --data-ascii "{ \"command1\": \"I Love You\", \"command2\": \"Charles\" }"` to test the Lambda function (arguments passed in may be different).

## Deployment 

### AWS Lambda Functionality
The core feature is the AWS Lambda function, which executes code in response to triggers without the need for provisioning or managing servers, offering scalability and efficiency.

- Sign up for an AWS account.
- Add an IAM User for credentials in AWS IAM web management console.
- Attach policies `lambdafullaccess` and `iamfullaccess`.
- Finish user creation, open user details, and go to security credentials.
- Generate access key.
- Store AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, and AWS_REGION in a `.env` file. 
- Add `.env` to `.gitignore`.
- Export the vars by using `export` in the terminal : 
```
set -a # automatically export all variables
source .env
set +a
```
This lets cargo lambda know which AWS account to deploy to.
- Build the project for release : `cargo lambda build --release`.
- Deploy the project for release : `cargo lambda deploy`.
- Login to AWS Lambda (make sure have the region correct on the top right bar) and check it is installed.

### AWS API Gateway Integration
Integrate with AWS API Gateway, providing a secure and scalable way to expose the Lambda function as a RESTful API. This enables easy interaction with the function over HTTP, making it accessible from anywhere.

- After AWS Lambda confirmation, connect the Lambda function with AWS API Gateway.
- Create a new API (keep default settings, REST API) then create a new resource (the URL path that will be appended to the API link).
- Select the resource, add method type `ANY` and select Lambda function.
- Turn off Lambda proxy integration if it's on.
- Deploy API by adding a new stage (another string that will be appended to the URL path).
- After clicking deploy, find `stages` and find the invoke URL.

API GATEWAY -> CREATE REST API -> CREATE RESOURCE -> CREATE ANY METHOD -> DEPLOY API -> GO TO STAGES —> CLICK DROPDOWN OF STAGES -> CLICK ANY OF THE TYPE LF REQUESTS (eg POST)

After everything is done, retrieve the link:
https://gvl7km0cu4.execute-api.us-east-1.amazonaws.com/tina/tinayiluo_resource

Test the api gateway by sending a curl post request:
```
curl -X POST https://gvl7km0cu4.execute-api.us-east-1.amazonaws.com/tina/tinayiluo_resource \
  -H 'content-type: application/json' \
  -d '{ "command1": "I Love You", "command2": "Charles" }'
```

Test the lambda fucntion without using the api gateway like so:
`cargo lambda invoke --remote <name of lambda function>`

## CI/CD Workflow
- After confirming the api gateway and lambda functions work, add AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, and AWS_REGION to gitlab/github secrets.
- The workflow includes running a `Makefile` to perform tasks such as installation (make install), testing (make test), code formatting (make format), linting (make lint), and an all-inclusive task (make all) using Github/Gitlab Actions. This automation streamlines the data processing and transformation process and enhances code quality.
- Use `.gitlab-ci.yml` to enable auto build, test, and deploy of the lambda function every time the repo is pushed.

## Usage 
The user could invoke the Lambda function by calling make aws-invoke (with the appropriate credentials) or send a request through the API Gateway.

## Diagram of Final Lambda Diagram Integration
![Screen_Shot_2024-02-03_at_6.39.26_PM](/uploads/d19fe8901472d2a13169734a0200d9db/Screen_Shot_2024-02-03_at_6.39.26_PM.png)

## Successful Test Example
![Screen_Shot_2024-02-03_at_9.28.20_PM](/uploads/f5db942a30effa3c78b7d8c294d52010/Screen_Shot_2024-02-03_at_9.28.20_PM.png)

![Screen_Shot_2024-02-03_at_9.57.21_PM](/uploads/0b255e49a57e0bfecb7eabdb92ebdfef/Screen_Shot_2024-02-03_at_9.57.21_PM.png)

![Screen_Shot_2024-02-03_at_10.38.24_PM](/uploads/0b4f5b7a32e11c09eb7fc2689dea1125/Screen_Shot_2024-02-03_at_10.38.24_PM.png)