mod routes;

use crate::schema::context::Context;
use std::net::SocketAddr;

pub async fn start(addr: impl Into<SocketAddr>, ctx: Context) {
    let routes = routes::make_routes(ctx);
    warp::serve(routes).run(addr).await;
}
