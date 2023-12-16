use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let health_route = warp::path!("health").and_then(health_handler);
    let wealth_route = warp::path!("wealth").and_then(wealth_handler);

    let routes = warp::get().and(
        health_route.with(warp::cors().allow_any_origin())
        .or( wealth_route.with(warp::cors().allow_any_origin())),);

    println!("Started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn health_handler() -> Result<impl Reply> {
    Ok("OK")
}
async fn wealth_handler() -> Result<impl Reply> {
    Ok("CASH")
}
