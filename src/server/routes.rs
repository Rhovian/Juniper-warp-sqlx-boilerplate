use serde_json::json;
use warp::filters::BoxedFilter;
use warp::reply::json;
use warp::{Filter, Rejection, Reply};

async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({
        "ok": true
    })))
}

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {
    let h = warp::path::end().and_then(health);

    h.boxed()
}
