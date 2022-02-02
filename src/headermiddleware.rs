use warp::Filter; 

const HEADERAUTH : &str = "x-auth";

pub fn check_header() -> impl Filter<Extract = ((),), warp::Rejection>{

  warp::any()
  .and(warp::header::<String>(HEADERAUTH))
  .and_then( |xauth: String| async move {
    if !xauth.ends_with(".exp.signature"){
      Ok(warp::reply::with_status("header not found", warp::http::StatusCode::BAD_REQUEST))
    }
    Ok<(), warp::Rejection>
  })
}