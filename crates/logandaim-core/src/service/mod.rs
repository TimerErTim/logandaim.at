use crate::ResultReport;

use async_trait::async_trait;
mod use_cases;

pub use crate::application::ExecutionContext;
use error_stack::IntoReport;
#[allow(unused_imports)]
pub use use_cases::*;

pub use crate::domain::konto::SessionAusgestellt;

#[async_trait]
pub trait UseCase<Output> {
    type Error: IntoReport + Send;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> ResultReport<Output, <Self::Error as IntoReport>::Context>;
}
