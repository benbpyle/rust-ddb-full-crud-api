mod data;

extern crate shared;

use crate::data::create_item;
use aws_sdk_dynamodb::Client;
use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestPayloadExt,
};
use serde_json::json;
use shared::models::dto::{BasicEntityCreateDto, BasicEntityViewDto};
use shared::models::entities::BasicEntity;
use tracing::error;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer};

async fn function_handler(
    table_name: &str,
    client: &Client,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    let body = event.payload::<BasicEntityCreateDto>()?;
    let mut return_body = json!("").to_string();
    let mut status_code = StatusCode::OK;

    match body {
        Some(v) => {
            let e: BasicEntity = v.into();
            let r = create_item(client, table_name, e).await;

            match r {
                Ok(v) => {
                    let dto = BasicEntityViewDto::from(v);
                    return_body = serde_json::to_string(&dto).unwrap();
                }
                Err(e) => {
                    error!("Error saving entity: {}", e);
                    status_code = StatusCode::BAD_REQUEST;
                    return_body = serde_json::to_string("Error saving entity").unwrap()
                }
            }
        }
        None => {
            status_code = StatusCode::BAD_REQUEST;
        }
    }

    let response = Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(return_body)
        .map_err(Box::new)?;
    Ok(response)
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

    run(service_fn(move |event: Request| async move {
        function_handler(table_name, shared_client, event).await
    }))
    .await
}
