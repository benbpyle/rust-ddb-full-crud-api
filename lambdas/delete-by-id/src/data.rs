use aws_sdk_dynamodb::{types::AttributeValue, Client};
use shared::models::errors::QueryError;

pub async fn delete_item(client: &Client, table_name: &str, id: &str) -> Result<(), QueryError> {
    let _ = client
        .delete_item()
        .key("id".to_string(), AttributeValue::S(id.to_string()))
        .table_name(table_name)
        .send()
        .await?;

    Ok(())
}
