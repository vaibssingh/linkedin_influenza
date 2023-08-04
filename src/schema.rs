use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
  pub page: Option<usize>,
  pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
  pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePostSchema {
  pub title: String,
  pub content: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub published: Option<bool>
}