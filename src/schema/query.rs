use crate::schema::context::Context;
use juniper::graphql_object;

#[derive(Default)]
pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn health(&self) -> bool {
        true
    }
    async fn getUsers(&self, ctx: &Context) -> bool {
        false
    }
}
