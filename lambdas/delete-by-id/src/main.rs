mod data;
extern crate shared;

use crate::data::delete_item;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use shared::models::errors::QueryError;
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
            let i: Result<(), QueryError> = delete_item(client, table_name, id).await;
            let mut status_code = 200;
            match i {
                Ok(_) => {}
                Err(_) => {
                    status_code = 404;
                }
            }
            let resp = shared::http::new_response("".to_string(), status_code);
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
