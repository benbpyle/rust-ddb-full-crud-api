mod data;
extern crate shared;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_dynamodb::Client;
use data::{get_item, update_item};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use shared::models::{
    dto::{BasicEntityPutDto, BasicEntityViewDto},
    errors::QueryError,
};
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
            // incoming
            let body = event.payload.body.unwrap();
            let id = o.into_mut();

            // operations on the models
            let found_item = get_item(client, table_name, id).await;

            match found_item {
                Ok(item) => {
                    let j: BasicEntityPutDto = serde_json::from_str(&body).unwrap();
                    let updated = update_item(client, table_name, item, j).await?;

                    // prep and return
                    let dto = BasicEntityViewDto::from(updated);
                    let resp =
                        shared::http::new_response(serde_json::to_string(&dto).unwrap(), 200);

                    Ok(resp)
                }
                Err(e) => match e {
                    QueryError::NotFound => {
                        let resp = shared::http::new_response(e.to_string(), 404);
                        Ok(resp)
                    }
                    _ => {
                        let resp = shared::http::new_response(e.to_string(), 400);
                        Ok(resp)
                    }
                },
            }
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
