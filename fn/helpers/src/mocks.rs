use http::{HeaderMap, HeaderValue};
use lambda_runtime::{Config, Context};

pub fn mock_config() -> Config {
    let conf = Config {
        function_name: "mockFunction".to_string(),
        memory: 32,
        version: "1.0.0".to_string(),
        log_stream: "logStream".to_string(),
        log_group: "logGroup".to_string(),
    };
    conf
}

pub fn mock_context() -> Context {
    let mut headers = HeaderMap::new();
    headers.insert(
        "lambda-runtime-aws-request-id",
        HeaderValue::from_static("my-id"),
    );
    headers.insert(
        "lambda-runtime-deadline-ms",
        HeaderValue::from_static("123"),
    );
    headers.insert(
        "lambda-runtime-invoked-function-arn",
        HeaderValue::from_static("arn::myarn"),
    );
    headers.insert(
        "lambda-runtime-trace-id",
        HeaderValue::from_static("arn::myarn"),
    );
    let context = Context::try_from(headers);
    context.unwrap()
}
