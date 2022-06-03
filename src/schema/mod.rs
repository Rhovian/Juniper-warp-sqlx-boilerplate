pub mod context;
mod query;

use context::Context;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

type Schema = RootNode<'static, query::Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn build_schema() -> Schema {
    Schema::new(query::Query, EmptyMutation::new(), EmptySubscription::new())
}
