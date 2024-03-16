use aws_sdk_dynamodb::{types::AttributeValue, Client};
use shared::models::entities::BasicEntity;
use shared::models::errors::QueryError;

pub async fn get_item(client: &Client, table_name: &str, id: &str) -> Result<BasicEntity, QueryError> {

    let output = client
        .get_item()
        .key("id".to_string(), AttributeValue::S(id.to_string()))
        .table_name(table_name)
        .send()
        .await?;

    match output.item {
        Some(item) => {
            let i: BasicEntity = serde_dynamo::from_item(item)?;
            Ok(i)
        },
        None => Err(QueryError::NotFound)
    }
}