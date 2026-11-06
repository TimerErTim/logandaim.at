//! Emit the OpenAPI document without a database.
//!
//! The spec needs only the *shape* of the app, never a live one: every port but the UoW
//! provider has an `App` default, and the provider is a stub that is never called.

use async_trait::async_trait;
use logandaim_api::openapi_service;
use logandaim_core::{
    App,
    application::IdentityProviderConfiguration,
    ports::RepositoryResult,
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

struct Unimplemented;

#[async_trait]
impl UnitOfWorkProvider for Unimplemented {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        unimplemented!()
    }
}

fn main() {
    let dummy_app = App::builder()
        .uow_provider(Box::new(Unimplemented))
        .konto(IdentityProviderConfiguration::neu(
            "logandaim.at",
            "logandaim-api",
        ))
        .build();

    let api_service = openapi_service(dummy_app, std::iter::empty::<String>());
    println!("{}", api_service.spec());
}
