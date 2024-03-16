mod data;
extern crate shared;

use aws_sdk_dynamodb::Client;
use data::get_items;
use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt,
};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer};

async fn handler(
    table_name: &str,
    client: &Client,
    request: Request,
) -> Result<impl IntoResponse, Error> {
    let last_key = request
        .query_string_parameters_ref()
        .and_then(|params| params.first("lk"))
        .unwrap_or_else(|| "")
        .to_string();
    let found_items = get_items(client, table_name, 10, last_key).await;
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&found_items.unwrap()).unwrap())
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
        handler(table_name, shared_client, event).await
    }))
    .await
}
