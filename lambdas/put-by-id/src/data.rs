use aws_sdk_dynamodb::{types::AttributeValue, Client};
use shared::models::dto::BasicEntityPutDto;
use shared::models::entities::BasicEntity;
use shared::models::errors::QueryError;

pub async fn get_item(
    client: &Client,
    table_name: &str,
    id: &str,
) -> Result<BasicEntity, QueryError> {
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
        }
        None => Err(QueryError::NotFound),
    }
}

pub async fn update_item(
    client: &Client,
    table_name: &str,
    mut item: BasicEntity,
    dto: BasicEntityPutDto,
) -> Result<BasicEntity, QueryError> {
    item.update_from_dto(dto);

    match client
        .put_item()
        .item("id".to_string(), AttributeValue::S(item.get_id()))
        .item("name".to_string(), AttributeValue::S(item.get_name()))
        .item(
            "description".to_string(),
            AttributeValue::S(item.get_description()),
        )
        .item(
            "entity_type".to_string(),
            AttributeValue::S(item.get_entity_type()),
        )
        .item(
            "updated_at".to_string(),
            AttributeValue::N(item.get_updated_at().to_string()),
        )
        .item(
            "created_at".to_string(),
            AttributeValue::N(item.get_created_at().to_string()),
        )
        .table_name(table_name)
        .send()
        .await
    {
        Ok(_) => Ok(item),
        Err(e) => Err(e.into()),
    }
}
