use crate::environment::Environment;
use shrinkwraprs::Shrinkwrap;

#[derive(Shrinkwrap, Clone)]
pub struct Context {
    env: Environment,
}

impl Context {
    pub async fn new(env: Environment) -> anyhow::Result<Self> {
        Ok(Self { env })
    }
}

impl juniper::Context for Context {}
