use error_stack::Report;
use http::StatusCode;
use logandaim_core::ThreadSafeError;
use logandaim_core::domain::schuss::SchussFehler;
use logandaim_core::domain::zeitraum::ZeitraumFehler;
use logandaim_core::ports::{ObjectStoreError, RepositoryError};
use logandaim_core::service::{
    AbmeldungFehler, AnmeldungFehler, BenutzerAnzeigenFehler, BenutzerLöschenFehler,
    ErneuerungFehler, PasswortVergessenFehler, PasswortWiederherstellenFehler,
    RegistrierungAbschliessenFehler, RegistrierungStartenFehler, SchussRegistrierenFehler,
    SessionPrüfenFehler, SystemadministratorBefördernFehler, WertungFehler,
};

use super::ValidationError;

/// Maps a typed error context to an HTTP status.
///
/// `None` means "not this layer" — [`status_from_report`] then walks nested
/// contexts (domain, repository) before defaulting to 500.
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
    mapped::<C, SchussFehler>(error)
        .or_else(|| mapped::<C, SchussRegistrierenFehler>(error))
        .or_else(|| mapped::<C, WertungFehler>(error))
        .or_else(|| mapped::<C, ZeitraumFehler>(error))
        .or_else(|| mapped::<C, AnmeldungFehler>(error))
        .or_else(|| mapped::<C, ErneuerungFehler>(error))
        .or_else(|| mapped::<C, AbmeldungFehler>(error))
        .or_else(|| mapped::<C, SessionPrüfenFehler>(error))
        .or_else(|| mapped::<C, RegistrierungStartenFehler>(error))
        .or_else(|| mapped::<C, RegistrierungAbschliessenFehler>(error))
        .or_else(|| mapped::<C, PasswortVergessenFehler>(error))
        .or_else(|| mapped::<C, PasswortWiederherstellenFehler>(error))
        .or_else(|| mapped::<C, BenutzerAnzeigenFehler>(error))
        .or_else(|| mapped::<C, BenutzerLöschenFehler>(error))
        .or_else(|| mapped::<C, SystemadministratorBefördernFehler>(error))
        .or_else(|| mapped::<C, RepositoryError>(error))
        .or_else(|| mapped::<C, ObjectStoreError>(error))
        .or_else(|| mapped::<C, ValidationError>(error))
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

impl HttpStatusMapping for ObjectStoreError {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            ObjectStoreError::Operation => Some(StatusCode::INTERNAL_SERVER_ERROR),
            ObjectStoreError::AlreadyDeleted => Some(StatusCode::NOT_FOUND),
        }
    }
}

impl HttpStatusMapping for ZeitraumFehler {
    fn http_status(&self) -> Option<StatusCode> {
        Some(StatusCode::BAD_REQUEST)
    }
}

impl HttpStatusMapping for SchussFehler {
    fn http_status(&self) -> Option<StatusCode> {
        Some(StatusCode::BAD_REQUEST)
    }
}

impl HttpStatusMapping for SchussRegistrierenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SchussRegistrierenFehler::Erstellung => None,
        }
    }
}

/// One impl per use case, because each one owns its own error enum.
///
/// Every `konto` refusal has exactly one HTTP meaning: `UngültigeAnfrage` is 400,
/// `NichtAngemeldet`/`ZugangsdatenFalsch`/`SessionUngültig` are 401, `NichtErlaubt` is 403,
/// `Gedrosselt` is 429, and a `*Laden`/`*Speichern`/`*Anlegen`/`*Pruefen` variant is a 500
/// fault of this deployment rather than a decision about the caller. A cause that reaches
/// here but is not in the list falls through to the repository mapping.
///
/// `NichtErlaubt` is 403, never 404: the caller proved who it is and is simply not allowed,
/// and pretending the record does not exist would only confuse a legitimate administrator
/// debugging their own account.
macro_rules! konto_status {
    ($fehler:ident { $($variante:ident => $status:expr),+ $(,)? }) => {
        impl HttpStatusMapping for $fehler {
            fn http_status(&self) -> Option<StatusCode> {
                Some(match self {
                    $(Self::$variante => $status),+
                })
            }
        }
    };
}

konto_status!(AnmeldungFehler {
    ZugangsdatenFalsch => StatusCode::UNAUTHORIZED,
    Gedrosselt => StatusCode::TOO_MANY_REQUESTS,
    KontoLaden => StatusCode::INTERNAL_SERVER_ERROR,
    BenutzerLaden => StatusCode::INTERNAL_SERVER_ERROR,
    PasswortPrüfen => StatusCode::INTERNAL_SERVER_ERROR,
    TokenErzeugen => StatusCode::INTERNAL_SERVER_ERROR,
    SessionSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(ErneuerungFehler {
    SessionUngültig => StatusCode::UNAUTHORIZED,
    Gedrosselt => StatusCode::TOO_MANY_REQUESTS,
    SessionLaden => StatusCode::INTERNAL_SERVER_ERROR,
    SessionSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
    TokenErzeugen => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(AbmeldungFehler {
    NichtAngemeldet => StatusCode::UNAUTHORIZED,
    SessionLaden => StatusCode::INTERNAL_SERVER_ERROR,
    SessionSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(SessionPrüfenFehler {
    NichtAngemeldet => StatusCode::UNAUTHORIZED,
    SessionLaden => StatusCode::INTERNAL_SERVER_ERROR,
    BenutzerLaden => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(RegistrierungStartenFehler {
    UngültigeAnfrage => StatusCode::BAD_REQUEST,
    Gedrosselt => StatusCode::TOO_MANY_REQUESTS,
    KontoPrüfen => StatusCode::INTERNAL_SERVER_ERROR,
    TokenErzeugen => StatusCode::INTERNAL_SERVER_ERROR,
    BestätigungSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(RegistrierungAbschliessenFehler {
    UngültigeAnfrage => StatusCode::BAD_REQUEST,
    Gedrosselt => StatusCode::TOO_MANY_REQUESTS,
    BestätigungLaden => StatusCode::INTERNAL_SERVER_ERROR,
    BestätigungSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
    PasswortSetzen => StatusCode::INTERNAL_SERVER_ERROR,
    BenutzerAnlegen => StatusCode::INTERNAL_SERVER_ERROR,
    KontoAnlegen => StatusCode::INTERNAL_SERVER_ERROR,
    TokenErzeugen => StatusCode::INTERNAL_SERVER_ERROR,
    SessionSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(PasswortVergessenFehler {
    UngültigeAnfrage => StatusCode::BAD_REQUEST,
    Gedrosselt => StatusCode::TOO_MANY_REQUESTS,
    TokenErzeugen => StatusCode::INTERNAL_SERVER_ERROR,
    KontoLaden => StatusCode::INTERNAL_SERVER_ERROR,
    WiederherstellungSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(PasswortWiederherstellenFehler {
    UngültigeAnfrage => StatusCode::BAD_REQUEST,
    PasswortSetzen => StatusCode::INTERNAL_SERVER_ERROR,
    WiederherstellungLaden => StatusCode::INTERNAL_SERVER_ERROR,
    WiederherstellungSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
    KontoSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
    SessionLaden => StatusCode::INTERNAL_SERVER_ERROR,
    SessionSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(BenutzerAnzeigenFehler {
    NichtAngemeldet => StatusCode::UNAUTHORIZED,
    NichtErlaubt => StatusCode::FORBIDDEN,
    BenutzerLaden => StatusCode::INTERNAL_SERVER_ERROR,
    KontoLaden => StatusCode::INTERNAL_SERVER_ERROR,
});

konto_status!(BenutzerLöschenFehler {
    UngültigeAnfrage => StatusCode::BAD_REQUEST,
    NichtAngemeldet => StatusCode::UNAUTHORIZED,
    PasswortFalsch => StatusCode::UNAUTHORIZED,
    PasswortPrüfen => StatusCode::INTERNAL_SERVER_ERROR,
    KontoLaden => StatusCode::INTERNAL_SERVER_ERROR,
    BenutzerLaden => StatusCode::INTERNAL_SERVER_ERROR,
    WiederherstellungLöschen => StatusCode::INTERNAL_SERVER_ERROR,
    BestätigungLöschen => StatusCode::INTERNAL_SERVER_ERROR,
    KontoLöschen => StatusCode::INTERNAL_SERVER_ERROR,
    SessionLöschen => StatusCode::INTERNAL_SERVER_ERROR,
    BenutzerLöschen => StatusCode::INTERNAL_SERVER_ERROR,
});

// Reached only from the startup bootstrap; a failure is a deployment fault.
konto_status!(SystemadministratorBefördernFehler {
    KontoLaden => StatusCode::INTERNAL_SERVER_ERROR,
    BenutzerLaden => StatusCode::INTERNAL_SERVER_ERROR,
    BenutzerSpeichern => StatusCode::INTERNAL_SERVER_ERROR,
});

impl HttpStatusMapping for WertungFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            WertungFehler::KeineSchüsse => Some(StatusCode::BAD_REQUEST),
            WertungFehler::NichtGefunden => Some(StatusCode::NOT_FOUND),
            WertungFehler::Laden => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn status<C: ThreadSafeError + HttpStatusMapping>(ctx: C) -> StatusCode {
        status_from_report(&Report::new(ctx))
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
    fn object_store_variants() {
        assert_eq!(
            status(ObjectStoreError::AlreadyDeleted),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(ObjectStoreError::Operation),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test_log::test]
    fn domain_construction_is_bad_request() {
        assert_eq!(
            status(ZeitraumFehler::EndeNichtNachBeginn),
            StatusCode::BAD_REQUEST
        );
    }

    #[test_log::test]
    fn konto_variants_have_one_meaning_each() {
        assert_eq!(
            status(AnmeldungFehler::ZugangsdatenFalsch),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            status(AnmeldungFehler::Gedrosselt),
            StatusCode::TOO_MANY_REQUESTS
        );
        assert_eq!(
            status(RegistrierungStartenFehler::UngültigeAnfrage),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            status(ErneuerungFehler::SessionUngültig),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            status(BenutzerAnzeigenFehler::NichtErlaubt),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            status(BenutzerLöschenFehler::PasswortFalsch),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            status(AbmeldungFehler::NichtAngemeldet),
            StatusCode::UNAUTHORIZED
        );
    }

    #[test_log::test]
    fn a_nested_repository_cause_is_used_when_the_use_case_maps_to_none() {
        // A use case that keeps the repository error in the chain still answers with its
        // own mapping; the fallback exists for the `None` cases above.
        let error = Report::new(SchussRegistrierenFehler::Erstellung)
            .change_context(RepositoryError::OperationFailed);
        assert_eq!(
            status_from_report(&error),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
