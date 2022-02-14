use warp::{Filter, http};
use warp::http::StatusCode;
use futures::future;
use crate::search::Search;

pub mod search;

fn json_body() -> impl Filter<Extract = (Search,), Error = warp::Rejection> + Clone {
    warp::query::query()
}

#[tokio::main]
async fn main() {

    let health = warp::path!("health")
        .map( || StatusCode::OK);

    let query = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("search"))
        .and(warp::path::end())
        .and(json_body())
        .and_then(update);

    let routes = query.or(health);


    let local = warp::serve(routes.clone())
        .run(([127, 0, 0, 1], 8080));
    let container = warp::serve(routes)
        .run(([0, 0, 0, 0], 8080));
    future::join(local, container).await;
}

async fn update(
    query: Search
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("searching {:?}", query.q);

    Ok(warp::reply::with_status(
        format!("Returning query results for {:?}" , query.q),
        http::StatusCode::CREATED,
    ))
}
