//! Repository tracing wrappers.
//!
//! Each wrapper holds a pointer to the parent [`UnitOfWorkImpl`] and delegates through the
//! matching accessor on every call.

use std::pin::Pin;

use async_trait::async_trait;
use tracing::instrument;

use crate::application::uow::UnitOfWorkImpl;
use crate::domain::konto::{
    BenutzerId, BestaetigungId, Konto, KontoId, NeueBestaetigung, NeueSession,
    NeueWiederherstellung, NeuerBenutzer, NeuesKonto, PasswortWiederherstellung, Rolle, Session,
    SessionId, SigningKey, SigningKeyId, TokenHash, WiederherstellungId,
};
use crate::domain::{EmailAdresse, NeuerSchuss, Schuss, SchussId};
use crate::ports::{
    BenutzerRepository, EmailBestaetigungRepository, KontoRepository,
    PasswortWiederherstellungRepository, RepositoryResult, SchussRepository, SessionRepository,
    SigningKeyRepository,
};

macro_rules! impl_instrumented_repo {
    (
        $wrapper:ident,
        $trait:ident,
        $accessor:ident,
        methods: [
            $(#[$meta:meta] fn $name:ident(&self $(, $arg:ident : $arg_ty:ty)*) -> $ret:ty;)*
        ]
    ) => {
        pub(crate) struct $wrapper {
            uow: Option<*const dyn UnitOfWorkImpl>,
        }

        impl $wrapper {
            pub(crate) fn new(uow: *const dyn UnitOfWorkImpl) -> Self {
                Self { uow: Some(uow) }
            }

            pub(crate) fn disabled() -> Self {
                Self { uow: None }
            }

            fn uow(&self) -> &dyn UnitOfWorkImpl {
                let uow = self
                    .uow
                    .expect("instrumented repository without unit of work");
                unsafe { &*uow }
            }
        }

        // SAFETY: `uow` points into a pinned `RepoStorage`; the address is stable until
        // the storage is consumed on commit/rollback.
        unsafe impl Send for $wrapper {}
        unsafe impl Sync for $wrapper {}

        #[async_trait]
        impl $trait for $wrapper {
            $(#[$meta]
            async fn $name(&self $(, $arg : $arg_ty)*) -> $ret {
                self.uow().$accessor().$name($($arg),*).await
            })*
        }
    };
}

impl_instrumented_repo!(
    InstrumentedSchussRepository,
    SchussRepository,
    schüsse,
    methods: [
        #[instrument(skip(self, schuss), level = "debug", err(Debug))]
        fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Schuss>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: SchussId) -> RepositoryResult<Schuss>;
        #[instrument(skip(self, ids), fields(anzahl = ids.len()), level = "trace", err(Debug))]
        fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Schuss>>;
    ]
);

impl_instrumented_repo!(
    InstrumentedBenutzerRepository,
    BenutzerRepository,
    benutzer,
    methods: [
        #[instrument(skip(self, benutzer), level = "debug", err(Debug))]
        fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<crate::domain::konto::Benutzer>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<crate::domain::konto::Benutzer>;
        #[instrument(skip(self, id), fields(id = ?id, rolle = rolle.as_str()), level = "debug", err(Debug))]
        fn setze_rolle(&self, id: BenutzerId, rolle: Rolle) -> RepositoryResult<()>;
        #[instrument(skip(self, id), fields(id = ?id), level = "debug", err(Debug))]
        fn delete(&self, id: BenutzerId) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedKontoRepository,
    KontoRepository,
    konten,
    methods: [
        #[instrument(skip(self, konto), level = "debug", err(Debug))]
        fn create(&self, konto: NeuesKonto) -> RepositoryResult<Konto>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: KontoId) -> RepositoryResult<Konto>;
        #[instrument(skip(self, email), fields(email = email.as_ref()), level = "trace", err(Debug))]
        fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Option<Konto>>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "trace", err(Debug))]
        fn find_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Konto>>;
        #[instrument(skip(self, konto), level = "debug", err(Debug))]
        fn speichere(&self, konto: &Konto) -> RepositoryResult<()>;
        #[instrument(skip(self, id), fields(id = ?id), level = "debug", err(Debug))]
        fn delete(&self, id: KontoId) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedSessionRepository,
    SessionRepository,
    sessions,
    methods: [
        #[instrument(skip(self, session), level = "debug", err(Debug))]
        fn create(&self, session: NeueSession) -> RepositoryResult<Session>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: SessionId) -> RepositoryResult<Session>;
        #[instrument(skip(self, hash), level = "trace", err(Debug))]
        fn find_by_refresh_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<Session>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "debug", err(Debug))]
        fn markiere_verbraucht(&self, id: SessionId, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
        #[instrument(skip(self, id), fields(id = ?id), level = "debug", err(Debug))]
        fn widerrufe(&self, id: SessionId, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "debug", err(Debug))]
        fn widerrufe_alle_fuer_benutzer(&self, benutzer_id: BenutzerId, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "debug", err(Debug))]
        fn delete_alle_fuer_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedSigningKeyRepository,
    SigningKeyRepository,
    signing_keys,
    methods: [
        #[instrument(skip(self, kid), fields(kid = kid.as_str()), level = "trace", err(Debug))]
        fn find_by_kid(&self, kid: &SigningKeyId) -> RepositoryResult<Option<SigningKey>>;
        #[instrument(skip(self, schluessel), level = "debug", err(Debug))]
        fn ensure(&self, schluessel: &SigningKey) -> RepositoryResult<()>;
        #[instrument(skip(self, kid), fields(kid = kid.as_str()), level = "debug", err(Debug))]
        fn ausser_betrieb_nehmen(&self, kid: &SigningKeyId, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedEmailBestaetigungRepository,
    EmailBestaetigungRepository,
    bestaetigungen,
    methods: [
        #[instrument(skip(self, bestaetigung), level = "debug", err(Debug))]
        fn create(&self, bestaetigung: NeueBestaetigung) -> RepositoryResult<crate::domain::konto::EmailBestaetigung>;
        #[instrument(skip(self, hash), level = "trace", err(Debug))]
        fn find_by_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<crate::domain::konto::EmailBestaetigung>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "debug", err(Debug))]
        fn consume(&self, id: BestaetigungId, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
        #[instrument(skip(self, email), fields(email = email.as_ref()), level = "debug", err(Debug))]
        fn invalidate_open_for_email(&self, email: &EmailAdresse, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
        #[instrument(skip(self, email), fields(email = email.as_ref()), level = "debug", err(Debug))]
        fn delete_alle_fuer_email(&self, email: &EmailAdresse) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedPasswortWiederherstellungRepository,
    PasswortWiederherstellungRepository,
    wiederherstellungen,
    methods: [
        #[instrument(skip(self, wiederherstellung), level = "debug", err(Debug))]
        fn create(&self, wiederherstellung: NeueWiederherstellung) -> RepositoryResult<PasswortWiederherstellung>;
        #[instrument(skip(self, hash), level = "trace", err(Debug))]
        fn find_by_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<PasswortWiederherstellung>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "debug", err(Debug))]
        fn consume(&self, id: WiederherstellungId, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
        #[instrument(skip(self, konto_id), fields(konto_id = ?konto_id), level = "debug", err(Debug))]
        fn invalidate_open_for_konto(&self, konto_id: KontoId, jetzt: chrono::DateTime<chrono::Utc>) -> RepositoryResult<()>;
        #[instrument(skip(self, konto_id), fields(konto_id = ?konto_id), level = "debug", err(Debug))]
        fn delete_alle_fuer_konto(&self, konto_id: KontoId) -> RepositoryResult<()>;
    ]
);

/// Pinned bundle: instrumented repos hold pointers into `uow` (field order keeps drop safe).
pub(crate) struct RepoStorage {
    pub(crate) schüsse: InstrumentedSchussRepository,
    pub(crate) benutzer: InstrumentedBenutzerRepository,
    pub(crate) konten: InstrumentedKontoRepository,
    pub(crate) sessions: InstrumentedSessionRepository,
    pub(crate) signing_keys: InstrumentedSigningKeyRepository,
    pub(crate) bestaetigungen: InstrumentedEmailBestaetigungRepository,
    pub(crate) wiederherstellungen: InstrumentedPasswortWiederherstellungRepository,
    pub(crate) uow: Box<dyn UnitOfWorkImpl>,
}

impl RepoStorage {
    pub(crate) fn new(uow: Box<dyn UnitOfWorkImpl>) -> Pin<Box<Self>> {
        let mut pinned = Box::pin(RepoStorage {
            schüsse: InstrumentedSchussRepository::disabled(),
            benutzer: InstrumentedBenutzerRepository::disabled(),
            konten: InstrumentedKontoRepository::disabled(),
            sessions: InstrumentedSessionRepository::disabled(),
            signing_keys: InstrumentedSigningKeyRepository::disabled(),
            bestaetigungen: InstrumentedEmailBestaetigungRepository::disabled(),
            wiederherstellungen: InstrumentedPasswortWiederherstellungRepository::disabled(),
            uow,
        });

        // SAFETY: `pinned` is boxed, so `uow`'s address is stable for the lifetime of the
        // storage. The wrappers are never moved out; they are dropped in place with it.
        unsafe {
            let this = pinned.as_mut().get_unchecked_mut();
            let uow_ptr = std::ptr::from_ref(this.uow.as_ref());
            this.schüsse = InstrumentedSchussRepository::new(uow_ptr);
            this.benutzer = InstrumentedBenutzerRepository::new(uow_ptr);
            this.konten = InstrumentedKontoRepository::new(uow_ptr);
            this.sessions = InstrumentedSessionRepository::new(uow_ptr);
            this.signing_keys = InstrumentedSigningKeyRepository::new(uow_ptr);
            this.bestaetigungen = InstrumentedEmailBestaetigungRepository::new(uow_ptr);
            this.wiederherstellungen =
                InstrumentedPasswortWiederherstellungRepository::new(uow_ptr);
        }

        pinned
    }
}
