mod errors;
use async_graphql::{EmptyMutation, EmptySubscription, ObjectType, Schema, SubscriptionType};
use events::model::Query as EventsQuery;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use crate::errors::ServerError;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Info {
    selection_set_graph_q_l: String,
    field_name: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventRequest {
    arguments: Option<Value>,
    info: Info,
}

async fn function_handler(event: LambdaEvent<EventRequest>) -> Result<Value, Error> {
    let (request, _context) = event.into_parts();

    let schema = Schema::build(EventsQuery, EmptyMutation, EmptySubscription).finish();
    let r = handle_query(schema, request).await?;
    Ok(r)
}

async fn handle_query<Query, Mutation, Subscription>(
    schema: Schema<Query, Mutation, Subscription>,
    request: EventRequest,
) -> Result<Value, ServerError>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    let gql_req = graphql_request(&request)?;
    println!("Query {:?}", gql_req);
    let response =
        serde_json::to_string(&schema.execute(gql_req).await).map_err(ServerError::from)?;

    let mut v: Value = serde_json::from_str(&response)?;
    let field_name = request.info.field_name;
    let result = v["data"][field_name].take();
    println!("result {:?}", result);
    Ok(result)
}

fn graphql_request(request: &EventRequest) -> Result<async_graphql::Request, ServerError> {
    // XXX: request doesn't have the entire query hence we can not pass request through async-graphql
    let query = "{ eventGet { id\n name } }".to_string();
    // let query = request.info.selection_set_graph_q_l;
    let mut req = async_graphql::Request::new(query);
    // req = req.operation_name(request.info.field_name);

    if let Some(arguments) = &request.arguments {
        let a = arguments.clone();
        let v = serde_json::from_value(a).unwrap_or_default();
        let variables = async_graphql::Variables::from_json(v);
        req = req.variables(variables);
    }

    Ok(req)
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

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod test {
    // use crate::Response;

    // #[test]
    // fn test_response_as_value() {
    //     let data = r#"
    //     {
    //         "id": "testid",
    //         "age": 43
    //     }"#;
    //     let v = Response::as_value(data).unwrap();
    //     assert_eq!(v["id"], "testid");
    //     assert_eq!(v["age"], 43);
    // }
}
