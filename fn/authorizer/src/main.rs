use std::env;

use lambda_runtime::{run, service_fn, Error, LambdaEvent};
mod types;
mod utils;
mod validation;

use crate::types::{AppSyncAuthorizerEvent, AppSyncAuthorizerResponse, ResolverContext};
use crate::utils::get_token;

async fn function_handler(
    event: LambdaEvent<AppSyncAuthorizerEvent>,
) -> Result<AppSyncAuthorizerResponse, Error> {
    let (data, _context) = event.into_parts();

    let authorization_token = data.authorization_token;

    let token = get_token(&authorization_token);
    println!("token {:?}", token);
    if token.is_none() {
        return Ok(AppSyncAuthorizerResponse {
            is_authorized: false,
            resolver_context: None,
            denied_field: None,
        });
    }
    let url: &str = &env::var("JWKS_URI").expect("JWKS_URI env not set");

    let jwt_data = &utils::fetch_jwks(url).await?;
    match validation::validate_token(jwt_data, token.unwrap()) {
        Ok(_) => {
            // Prepare the response
            let resp = AppSyncAuthorizerResponse {
                is_authorized: true,
                resolver_context: Some(ResolverContext {
                    userid: "test-user-id".to_string(),
                    info: format!("token {}", token.unwrap()),
                    more_info: "more info".to_string(),
                }),
                denied_field: None,
            };

            println!("=== success!");
            Ok(resp)
        }
        Err(_) => Ok(AppSyncAuthorizerResponse {
            is_authorized: false,
            resolver_context: None,
            denied_field: None,
        }),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let func = service_fn(function_handler);
    run(func).await
}

#[cfg(test)]
mod test {
    use helpers::mocks::mock_context;
    use lambda_runtime::LambdaEvent;

    use crate::{
        function_handler,
        types::{AppSyncAuthorizerEvent, RequestContext},
    };

    #[tokio::test]
    async fn test_handler_should_return_authorized_response() {
        let event = LambdaEvent::new(
            AppSyncAuthorizerEvent {
                authorization_token: "Bearer token123".to_string(),
                request_context: RequestContext {
                    api_id: "apiid".to_string(),
                },
            },
            mock_context(),
        );
        let result = function_handler(event);
        let response = result.await.unwrap();
        assert_eq!(response.is_authorized, true);
        let info = response.resolver_context.unwrap().info;
        assert_eq!(info, "token token123");
    }
}
