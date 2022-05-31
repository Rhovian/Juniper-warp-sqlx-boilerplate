use juniper::graphql_object;

#[derive(Default)]
pub struct Query;

#[graphql_object]
impl Query {
    async fn health(&self) -> bool {
        true
    }
    async fn userWithEMail(&self, email: String) -> String {
        email.to_string()
    }
}
