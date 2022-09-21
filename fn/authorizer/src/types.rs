use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    // TODO: implement
    // "requestContext": {
    //       "apiId": "3q776ansnnakxjk33yin3xwziy",
    //       "accountId": "101567964829",
    //       "requestId": "3b45143d-ee88-4bf1-9115-7faeab13fef6",
    //       "queryString": "query MyQuery {\n  eventGet(id: \"id\") {\n\n    id\n    name\n  }\n}\n",
    //       "operationName": "MyQuery",
    //       "variables": {}
    //   }
    pub api_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResolverContext {
    pub userid: String,
    pub info: String,
    pub more_info: String,
}

// Incoming AppSync Authorizer Event
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSyncAuthorizerEvent {
    pub authorization_token: String,
    pub request_context: RequestContext,
}

// Outgoing AppSync Authorizer Event
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSyncAuthorizerResponse {
    pub is_authorized: bool,
    pub resolver_context: Option<ResolverContext>,
    pub denied_field: Option<Box<[String]>>,
}
