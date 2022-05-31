use juniper::graphql_object;

#[derive(Default)]
pub struct HealthQuery;

#[graphql_object]
impl HealthQuery {
    async fn health(&self) -> bool {
        true
    }
}
