mod cases;

use logandaim_core::{
    App,
    application::{AppBuilder, SetUowProvider},
};
use logandaim_fakes::FakeUnitOfWorkProvider;

pub async fn base_app_builder() -> AppBuilder<SetUowProvider> {
    let uow_provider = FakeUnitOfWorkProvider::empty();
    App::builder().uow_provider(Box::new(uow_provider))
}
