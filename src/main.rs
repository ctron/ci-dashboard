use std::convert::Infallible;
use warp::Filter;

use hubcaps::{Credentials, Github};
use hubcaps::pulls::Pull;

async fn github_pr_state(
    org: String,
    repo_name: String,
    pr: u64,
) -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&"ok"))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let state = warp::path("state")
        .and(warp::path::param())
        .and(warp::path::param())
        .and(warp::path::param())
        .and_then(github_pr_state);

    let dist = warp::fs::dir("dist");

    let routes = warp::get().and(hello.or(state).or(dist));

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
