use crate::response::{PostData, PostListResponse, PostResponse, SinglePostResponse};
use crate::{
  error::Error::*, model::PostModel, schema::CreatePostSchema, Result,
};
use chrono::prelude::*;
use futures::StreamExt;