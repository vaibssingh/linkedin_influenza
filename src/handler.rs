use crate::{
  db::DB,
  response::GenericResponse,
  schema::{CreatePostSchema, FilterOptions},
  WebResult,
};

use warp::{http::StatusCode, reject, reply::json, reply::with_status, Reply};

pub async fn health_checker_handler() -> WebResult<impl Reply> {
  const MESSAGE: &str = "LinkedIn App lol";

  let response_json = &GenericResponse {
      status: "success".to_string(),
      message: MESSAGE.to_string(),
  };
  Ok(json(response_json))
}

pub async fn posts_list_handler(opts: FilterOptions, db:DB) -> WebResult<impl Reply> {
  let limit = opts.limit.unwrap_or(10) as i64;
  let page = opts.page.unwrap_or(1) as i64;

  let result_json = db
    .fetch_posts(limit, page)
    .await
    .map_err(|e| reject::custom(e))?;

  Ok(json(&result_json))
}

pub async fn create_post_handler(body: CreatePostSchema, db:DB) -> WebResult<impl Reply> {
  let post = db.create_posts(&body).await.map_err(|e| reject::custom(e))?;

  Ok(with_status(json(&post), StatusCode::CREATED))
}

pub async fn get_post_handler(id: String, db:DB) -> WebResult<impl Reply> {
  let post = db.get_post(&id).await.map_err(|e| reject::custom(e))?;

  let error_response = GenericResponse {
    status: "fail".to_string(),
    message: format!("Note with ID: {} not found", id)
  };

  if post.is_none() {
    return Ok(with_status(json(&error_response), StatusCode::NOT_FOUND));
  }

  Ok(with_status(json(&post), StatusCode::OK))
}