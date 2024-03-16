mod data;
extern crate shared;

use crate::data::get_item;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use shared::models::dto::BasicEntityViewDto;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer};

async fn function_handler(
    table_name: &str,
    client: &Client,
    mut event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    match event.payload.path_parameters.entry("id".to_string()) {
        Occupied(o) => {
            let id = o.into_mut();
            let item = get_item(client, table_name, id).await?;
            let dto = BasicEntityViewDto::from(item);
            let resp = shared::http::new_response(serde_json::to_string(&dto).unwrap(), 200);
            Ok(resp)
        }
        Vacant(_) => {
            let resp = shared::http::new_response("".to_string(), 404);
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let filtered_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .json()
        .with_target(true)
        .with_file(true)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(filtered_layer).init();
    let is_local = std::env::var("IS_LOCAL").unwrap_or("false".to_string());
    let client = shared::clients::lambda_ddb_client::new_client(is_local).await;
    let table_name = &std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let shared_client = &client;

    run(service_fn(
        move |event: LambdaEvent<ApiGatewayProxyRequest>| async move {
            function_handler(table_name, shared_client, event).await
        },
    ))
    .await
}
