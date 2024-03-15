mod data;

extern crate shared;

use crate::data::create_item;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_sdk_dynamodb::Client;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use shared::models::dto::{BasicEntityCreateDto, BasicEntityViewDto};
use shared::models::entities::BasicEntity;
use tracing::metadata::LevelFilter;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, Layer};

async fn function_handler(
    table_name: &str,
    client: &Client,
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let body = event.payload.body.unwrap();
    let j: BasicEntityCreateDto = serde_json::from_str(&body).unwrap();
    let e: BasicEntity = j.into();
    let r = create_item(client, table_name, e).await;

    match r {
        Ok(v) => {
            info!("(Response)={:?}", v);
            let id = v.get_id();
            let dto = BasicEntityViewDto::from(v);
            Ok(shared::http::new_content_created_response(
                serde_json::to_string(&dto).unwrap(),
                201,
                id,
            ))
        }
        Err(e) => {
            error!("(Error)={:?}", e);
            Ok(shared::http::new_response("".to_string(), 500))
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
