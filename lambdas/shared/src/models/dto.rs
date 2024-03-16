use crate::models::entities::BasicEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use svix_ksuid::{Ksuid, KsuidLike};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicEntityViewPaginated {
    #[serde(deserialize_with = "null_to_default")]
    pub last_evaluated_key: String,
    pub entities: Vec<BasicEntityViewDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicEntityViewDto {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(serialize_with = "serialize_dt")]
    pub created_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_dt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct BasicEntityCreateDto {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct BasicEntityPutDto {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl BasicEntityViewPaginated {
    pub fn new(last_evaluated_key: String, entities: Vec<BasicEntityViewDto>) -> Self {
        Self {
            last_evaluated_key,
            entities,
        }
    }
}

impl Into<BasicEntity> for BasicEntityCreateDto {
    fn into(self) -> BasicEntity {
        let ksuid = Ksuid::new(None, None);
        let dt = Utc::now();
        let timestamp: i64 = dt.timestamp();

        BasicEntity::new(
            ksuid.to_string(),
            self.name,
            self.description,
            "BasicEntity".to_string(),
            timestamp,
            timestamp,
        )
    }
}

impl From<BasicEntity> for BasicEntityViewDto {
    fn from(value: BasicEntity) -> Self {
        let created_at = DateTime::from_timestamp(value.get_created_at(), 0).unwrap();
        let updated_at = DateTime::from_timestamp(value.get_updated_at(), 0).unwrap();

        BasicEntityViewDto {
            id: value.get_id(),
            name: value.get_name(),
            description: value.get_description(),
            created_at,
            updated_at,
        }
    }
}

pub fn serialize_dt<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    dt.format("%+").to_string().serialize(serializer)
}
