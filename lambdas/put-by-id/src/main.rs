mod data;
extern crate shared;
use aws_sdk_dynamodb::Client;
use data::get_item;
use data::update_item;
use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt, RequestPayloadExt,
};
use serde_json::json;
use shared::models::{
    dto::{BasicEntityPutDto, BasicEntityViewDto},
    errors::QueryError,
};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer};

async fn function_handler(
    table_name: &str,
    client: &Client,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    let id = event
        .path_parameters_ref()
        .and_then(|params| params.first("id"))
        .unwrap();

    let mut status_code = StatusCode::OK;
    let mut body = json!("").to_string();
    let found_item = get_item(client, table_name, id).await;

    match found_item {
        Ok(item) => {
            let j: BasicEntityPutDto = event.payload::<BasicEntityPutDto>().unwrap().unwrap();
            let updated = update_item(client, table_name, item, j).await?;

            // prep and return
            let dto = BasicEntityViewDto::from(updated);
            body = serde_json::to_string(&dto).unwrap();
        }
        Err(e) => match e {
            QueryError::NotFound => {
                status_code = StatusCode::NOT_FOUND;
            }
            _ => {
                status_code = StatusCode::BAD_REQUEST;
            }
        },
    }

    let response = Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
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
