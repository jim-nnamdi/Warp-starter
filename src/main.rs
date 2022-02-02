mod posts;
mod collections;

use warp::Filter;

const TEMPLATEDIRECTORY : &str = "templates/";

#[tokio::main]
async fn main() {

    let content = warp::fs::dir(TEMPLATEDIRECTORY);

    let root = warp::get().and(warp::path::end()).and(warp::fs::file(format!("{}/index.html", TEMPLATEDIRECTORY)));

    let webroutes = content.or(root).or(all_posts());

    warp::serve(webroutes).run(([127,0,0,1], 7900)).await;
}