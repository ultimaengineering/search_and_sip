use warp::Filter;
use warp::http::StatusCode;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let health = warp::path!("health")
        .map( || StatusCode::OK);

    let routes = health.or(hello)
        .with(warp::cors().allow_any_origin());


    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}