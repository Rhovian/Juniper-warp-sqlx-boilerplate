mod health;
use juniper::{EmptyMutation, EmptySubscription, RootNode};

type Schema = RootNode<'static, health::HealthQuery, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> Schema {
    Schema::new(
        health::HealthQuery,
        EmptyMutation::new(),
        EmptySubscription::new(),
    )
}
