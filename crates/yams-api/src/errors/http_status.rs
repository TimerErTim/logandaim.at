use error_stack::Report;
use http::StatusCode;
use yams_core::ThreadSafeError;
use yams_core::ports::RepositoryError;

use super::ValidationError;

/// Maps a typed error context to an HTTP status.
///
/// `None` means "not this layer" — [`status_from_report`] then walks nested
/// contexts (repository) before defaulting to 500.
pub trait HttpStatusMapping {
    fn http_status(&self) -> Option<StatusCode> {
        None
    }
}

pub fn status_from_report<C: ThreadSafeError + HttpStatusMapping>(error: &Report<C>) -> StatusCode {
    error
        .request_value::<StatusCode>()
        .next()
        .or_else(|| error.downcast_ref::<StatusCode>().copied())
        .or_else(|| error.current_context().http_status())
        .or_else(|| nested_http_status(error))
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

fn mapped<C: ThreadSafeError, T: ThreadSafeError + HttpStatusMapping>(
    error: &Report<C>,
) -> Option<StatusCode> {
    error
        .downcast_ref::<T>()
        .and_then(HttpStatusMapping::http_status)
}

fn nested_http_status<C: ThreadSafeError>(error: &Report<C>) -> Option<StatusCode> {
    mapped::<C, RepositoryError>(error).or_else(|| mapped::<C, ValidationError>(error))
}

impl HttpStatusMapping for ValidationError {
    fn http_status(&self) -> Option<StatusCode> {
        Some(StatusCode::BAD_REQUEST)
    }
}

impl HttpStatusMapping for RepositoryError {
    fn http_status(&self) -> Option<StatusCode> {
        Some(match self {
            RepositoryError::NotFound => StatusCode::NOT_FOUND,
            RepositoryError::VersionMismatch { .. } | RepositoryError::Conflict => {
                StatusCode::CONFLICT
            }
            RepositoryError::Permission => StatusCode::FORBIDDEN,
            RepositoryError::Connection => StatusCode::SERVICE_UNAVAILABLE,
            RepositoryError::OperationFailed
            | RepositoryError::Storage
            | RepositoryError::Data
            | RepositoryError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status<C: ThreadSafeError + HttpStatusMapping>(ctx: C) -> StatusCode {
        status_from_report(&Report::new(ctx))
    }

    fn wrapped<
        Inner: ThreadSafeError + HttpStatusMapping,
        Outer: ThreadSafeError + HttpStatusMapping,
    >(
        inner: Inner,
        outer: Outer,
    ) -> StatusCode {
        status_from_report(&Report::new(inner).change_context(outer))
    }

    #[test_log::test]
    fn attached_status_wins() {
        let error = Report::new(RepositoryError::NotFound).attach_opaque(StatusCode::BAD_REQUEST);
        assert_eq!(status_from_report(&error), StatusCode::BAD_REQUEST);
    }

    #[test_log::test]
    fn validation_is_bad_request() {
        assert_eq!(status(ValidationError), StatusCode::BAD_REQUEST);
    }

    #[test_log::test]
    fn repository_variants() {
        assert_eq!(status(RepositoryError::NotFound), StatusCode::NOT_FOUND);
        assert_eq!(
            status(RepositoryError::VersionMismatch {
                expected: 1,
                actual: Some(2)
            }),
            StatusCode::CONFLICT
        );
        assert_eq!(status(RepositoryError::Conflict), StatusCode::CONFLICT);
        assert_eq!(status(RepositoryError::Permission), StatusCode::FORBIDDEN);
        assert_eq!(
            status(RepositoryError::Connection),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            status(RepositoryError::OperationFailed),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test_log::test]
    fn repository_status_is_preserved_through_layers() {
        assert_eq!(
            wrapped(RepositoryError::NotFound, ValidationError),
            StatusCode::BAD_REQUEST
        );
    }
}
