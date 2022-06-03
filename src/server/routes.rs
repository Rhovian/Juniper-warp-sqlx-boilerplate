use crate::schema;
use schema::context::Context;
use serde_json::json;
use warp::filters::BoxedFilter;
use warp::http::Response;
use warp::reply::json;
use warp::{Filter, Rejection, Reply};

async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({
        "ok": true
    })))
}

async fn home() -> Result<impl Reply, Rejection> {
    Ok(Response::builder()
        .header("content-type", "text/html")
        .body("<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"))
}

pub(super) fn make_routes(ctx: Context) -> BoxedFilter<(impl Reply,)> {
    let state = warp::any().map(move || ctx.clone());

    let graphql_filter = juniper_warp::make_graphql_filter(schema::build_schema(), state.boxed());

    let home = warp::path::end().and_then(home);
    let health = warp::path("health").and_then(health);
    let graphiql = warp::path("graphiql").and(juniper_warp::graphiql_filter("/graphql", None));

    home.or(health)
        .or(graphiql)
        .or(warp::path("graphql").and(graphql_filter))
        .boxed()
}
