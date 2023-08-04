use chrono::prelude::*;
use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostModel {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub content: String,
  pub published: Option<bool>,
  #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
  pub created_at: DateTime<Utc>,
  #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
  pub updated_at: DateTime<Utc>
}