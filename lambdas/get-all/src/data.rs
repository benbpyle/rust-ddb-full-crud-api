use aws_sdk_dynamodb::Client;
use shared::models::dto::BasicEntityViewDto;
use shared::models::errors::QueryError;
use shared::models::{dto::BasicEntityViewPaginated, entities::BasicEntity};
use std::collections::HashMap;

use tracing::error;

pub async fn get_items(
    client: &Client,
    table_name: &str,
    limit: i32,
    last_key: String,
) -> Result<BasicEntityViewPaginated, QueryError> {
    let mut key = None;

    if last_key != "" {
        let mut evaluated_key: HashMap<String, aws_sdk_dynamodb::types::AttributeValue> =
            HashMap::new();
        evaluated_key.insert(
            "id".to_string(),
            aws_sdk_dynamodb::types::AttributeValue::S(last_key),
        );
        key = Some(evaluated_key);
    }

    let output = client
        .scan()
        .set_exclusive_start_key(key)
        .limit(limit)
        .table_name(table_name)
        .send()
        .await?;

    match output.items {
        Some(item) => {
            let mut entities = Vec::new();

            for i in item {
                let entity: Result<BasicEntity, serde_dynamo::Error> = serde_dynamo::from_item(i);
                match entity {
                    Ok(entity) => {
                        entities.push(BasicEntityViewDto::from(entity));
                    }
                    Err(e) => {
                        error!("(Error)={:?}", e);
                    }
                }
            }

            let mut last_key: String = String::new();

            if output.last_evaluated_key.is_some() {
                let key = output.last_evaluated_key.unwrap();
                let key_value = key.get("id").unwrap();
                let string_value = key_value.as_s().unwrap().to_string();

                last_key = string_value;
            }

            Ok(BasicEntityViewPaginated::new(last_key, entities))
        }
        None => {
            let entities = Vec::new();
            Ok(BasicEntityViewPaginated::new("".to_string(), entities))
        }
    }
}
