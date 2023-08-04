use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
  pub status: String,
  pub message: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct PostResponse {
  pub id: String,
  pub title: String,
  pub content: String,
  pub published: bool,
  pub createdAt: DateTime<Utc>,
  pub updatedAt: DateTime<Utc>
}

#[derive(Serialize, Debug)]
pub struct PostData {
  pub post: PostResponse
}

#[derive(Serialize, Debug)]
pub struct SinglePostResponse {
  pub status: String,
  pub data: PostData
}

#[derive(Serialize, Debug)]
pub struct PostListResponse {
  pub status: String,
  pub results: usize,
  pub posts: Vec<PostResponse>
}