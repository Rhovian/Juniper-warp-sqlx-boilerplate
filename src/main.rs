mod environment;
mod schema;
mod server;

use dotenv::dotenv;
use std::env;

extern crate dotenv;
extern crate log;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    pretty_env_logger::init();

    dotenv().ok();

    let db = dotenv::var("DATABASE_URL").unwrap();

    let env = environment::Environment::new(&db).await?;
    let context = schema::context::Context::new(env).await?;

    server::start(([127, 0, 0, 1], 3030), context).await;

    Ok(())
}
