use warp::Filter;
use serde_json::{json, Value};
use warp::reply::Json;

pub fn all_posts() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

  let post_base_url = warp::path("posts");

  let return_all_posts = post_base_url
  .and(warp::get())
  .and(warp::path::end())
  .and_then(posts_list);

  let return_single_post = post_base_url
  .and(warp::get())
  .and(warp::path::param())
  .and_then(single_post);
}

async fn posts_list() -> Result<Json, warp::Rejection>{
  let posts = json!([
    {"id": 1, "title": "Post one", "author": "metro"},
    {"id": 2, "title": "Post two", "author": "dayoo"}
  ]);

  let posts = warp::reply::json(&posts);
  Ok(posts)
}

async fn single_post() -> Result<Json, warp::Rejection>{
  let post = json!([
    {"id": 1, "title": "Post one", "author": "metro"}
  ]);

  let post = warp::reply::json(&post);
  Ok(post)
}