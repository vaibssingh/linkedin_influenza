use crate::response::{PostData, PostListResponse, PostResponse, SinglePostResponse};
use crate::{error::Error::*, model::PostModel, schema::CreatePostSchema, Result};
use chrono::prelude::*;
use futures::StreamExt;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::{FindOptions, IndexOptions};
use mongodb::{bson, options::ClientOptions, Client, Collection, IndexModel};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DB {
    pub posts_collection: Collection<PostModel>,
    pub collection: Collection<Document>,
}

impl DB {
    pub async fn init() -> Result<Self> {
        let mongodb_uri: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_name: String =
            std::env::var("MONGODB_INITDB_DATABASE").expect("MONGODB_INITDB_DATABASE is not set");
        let mongodb_posts_collection: String =
            std::env::var("MONGODB_POSTS_COLLECTION").expect("MONGODB_POSTS_COLLECTION is not set");

        let mut client_options = ClientOptions::parse(mongodb_uri).await?;
        client_options.app_name = Some(database_name.to_string());

        let client = Client::with_options(client_options)?;
        let database = client.database(database_name.as_str());

        let posts_collection = database.collection(mongodb_posts_collection.as_str());
        let collection = database.collection::<Document>(mongodb_posts_collection.as_str());

        println!("✅ Database connected successfully");

        Ok(Self {
            posts_collection,
            collection,
        })
    }

    fn doc_to_post(&self, post: &PostModel) -> Result<PostResponse> {
        let post_response = PostResponse {
            id: post.id.to_hex(),
            title: post.title.to_owned(),
            content: post.content.to_owned(),
            published: post.published.unwrap(),
            createdAt: post.createdAt,
            updatedAt: post.updatedAt,
        };

        Ok(post_response)
    }

    pub async fn fetch_posts(&self, limit: i64, page: i64) -> Result<PostListResponse> {
        let find_options = FindOptions::builder()
            .limit(limit)
            .skip(u64::try_from((page - 1) * limit).unwrap())
            .build();

        let mut cursor = self
            .posts_collection
            .find(None, find_options)
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<PostResponse> = Vec::new();

        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_post(&doc.unwrap())?);
        }

        let json_posts_list = PostListResponse {
          status: "success".to_string(),
          results: json_result.len(),
          posts: json_result
        };

        Ok(json_posts_list)
    }

    pub async fn create_posts(&self, body: &CreatePostSchema) -> Result<Option<SinglePostResponse>> {
      let published = body.published.to_owned().unwrap_or(false);
      let serialized_data = bson::to_bson(&body).map_err(MongoSerializeBsonError)?;
      let document = serialized_data.as_document().unwrap();
      let options = IndexOptions::builder().unique(true).build();
      let index = IndexModel::builder()
          .keys(doc! {"title": 1})
          .options(options)
          .build();

      self.posts_collection
        .create_index(index, None)
        .await
        .expect("error creating index!");

      let datetime = Utc::now();

      let mut doc_with_dates = doc! {"createdAt": datetime, "updatedAt": datetime, "published": published};
      doc_with_dates.extend(document.clone());

      let insert_result = self
        .collection
        .insert_one(&doc_with_dates, None)
        .await
        .map_err(|e| {
          if e.to_string().contains("E11000 duplicate key error collection") {
            return MongoDuplicateError(e);
          }
          return MongoQueryError(e);
        })?;

        let new_id = insert_result
          .inserted_id
          .as_object_id()
          .expect("issue with new _id");

        let post_doc = self
          .posts_collection
          .find_one(doc! {"_id": new_id}, None)
          .await
          .map_err(MongoQueryError)?;

        if post_doc.is_none() {
          return Ok(None);
        }

        let post_response = SinglePostResponse {
          status: "success".to_string(),
          data: PostData { 
            post: self.doc_to_post(&post_doc.unwrap()).unwrap()
           }
        };

        Ok(Some(post_response))
    }

    pub async fn get_post(&self, id: &str) -> Result<Option<SinglePostResponse>> {
      let oid = ObjectId::from_str(id).map_err(|_| InvalidIDError(id.to_owned()))?;

      let post_doc = self
        .posts_collection
        .find_one(doc! {"_id": oid}, None)
        .await
        .map_err(MongoQueryError)?;

      if post_doc.is_none() {
        return Ok(None);
      }

      let post_response = SinglePostResponse {
        status: "success".to_string(),
        data: PostData { 
          post: self.doc_to_post(&post_doc.unwrap()).unwrap()
         }
      };

      Ok(Some(post_response))
    }

}
