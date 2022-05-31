mod query;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

type Schema = RootNode<'static, query::Query, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> Schema {
    Schema::new(query::Query, EmptyMutation::new(), EmptySubscription::new())
}
