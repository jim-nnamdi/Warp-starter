use warp::Filter; 
use serde_json::{json, Value};
use warp::reply::Json; 

pub fn all_collections() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {

  let collections_base_url = warp::path("collections");

  let return_all_collections = collections_base_url
  .and(warp::get())
  .and(warp::path::end())
  .and_then(all_collects);

  let return_single_collection = collections_base_url
  .and(warp::get())
  .and(warp::path::param())
  .and_then(single_collection);

  let return_create_collection = collections_base_url
  .and(warp::post())
  .and(warp::body::json())
  .and_then(create_collection);

  return_all_collections.or(return_single_collection).or(create_collections);
}

async fn all_collects() -> Result<Json, warp::Rejection> {
  let collections = json!([
    {"id": 1, "title": "first collection"},
    {"id": 2, "title": "second collection"}
  ]);

  let collections = warp::reply::json(&collections);
  Ok(collections);
}

async fn single_collection(id: i64) -> Result<Json, warp::Rejection>{
  let collection = json!([
    {"id": 1, "title": "first collection"}
  ]);

  let collection = warp:reply:json(&collection);
  Ok(collection);
} 

async fn create_collection(data: Value) -> Result<Json, warp::Rejection>{
  let collection = Value;
  let collection = warp::reply::json(&collection);
  Ok(collection);
}