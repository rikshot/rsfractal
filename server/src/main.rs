use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let dir_filter = warp::get().and(warp::path("docs"))
        .and(warp::fs::dir("docs/"))
        .with(warp::trace::request());

    let mut headers = HeaderMap::new();
    headers.insert("Cross-Origin-Embedder-Policy", HeaderValue::from_static("require-corp"));
    headers.insert("Cross-Origin-Opener-Policy", HeaderValue::from_static("same-origin"));

    let index_filter = warp::get().and(warp::path::end())
        .and(warp::fs::file("docs/index.html").with(warp::reply::with::headers(headers)))
        .with(warp::trace::request());

    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string()).parse().expect("PORT must be a string");
    warp::serve(index_filter.or(dir_filter))
        .run(([0, 0, 0, 0], port))
        .await;
}
