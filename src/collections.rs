use warp::Filter; 
use serde_json::{json, Value};
use warp::reply::Json; 

pub fn all_collections() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
  
}