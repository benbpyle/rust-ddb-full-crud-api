use aws_sdk_dynamodb::{types::AttributeValue, Client};
use shared::models::errors::QueryError;

pub async fn delete_item(client: &Client, table_name: &str, id: &str) -> Result<(), QueryError> {
    let output = client
        .delete_item()
        .key("id".to_string(), AttributeValue::S(id.to_string()))
        .table_name(table_name)
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
        .send()
        .await?;

    match output.attributes() {
        Some(_) => Ok(()),
        None => Err(QueryError::NotFound),
    }
}
