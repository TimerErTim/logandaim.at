use crate::ResultReport;

use async_trait::async_trait;
mod access;
mod use_cases;

pub use crate::application::ExecutionContext;
pub use access::*;
use error_stack::IntoReport;
#[allow(unused_imports)]
pub use use_cases::*;

/// One write operation, performed inside its own unit of work.
///
/// Reads are not use cases: the API facade runs them as one-off `execute_fn` closures.
/// `SessionPrüfen` is the one exception — verifying an access token decides an identity.
#[async_trait]
pub trait UseCase<Output> {
    type Error: IntoReport + Send;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> ResultReport<Output, <Self::Error as IntoReport>::Context>;
}
