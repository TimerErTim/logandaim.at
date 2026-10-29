//! Repository tracing wrappers.
//!
//! Each wrapper holds a pointer to the parent [`UnitOfWorkImpl`] and delegates through the
//! matching accessor on every call.

use std::pin::Pin;

use async_trait::async_trait;
use tracing::instrument;

use crate::application::uow::UnitOfWorkImpl;
use crate::application::uow::Versioned;
use crate::domain::konto::{
    Benutzer, BenutzerId, BestätigungId, EmailBestätigung, Konto, KontoId, NeueBestätigung,
    NeuePasswortWiederherstellung, NeueSession, NeuerBenutzer, NeuesKonto,
    PasswortWiederherstellung, PasswortWiederherstellungId, SecretTokenHash, Session, SessionId,
};
use crate::domain::{
    Anlage, AnlageId, EmailAdresse, NeueAnlage, NeueRunde, NeuerSchuss, ParcoursId, Runde, RundeId,
    Schuss, SchussId, ZielId,
};
use crate::ports::{
    AnlageRepository, BenutzerRepository, EmailBestätigungRepository, KontoRepository,
    PasswortWiederherstellungRepository, RepositoryResult, RundeRepository, SchussRepository,
    SessionRepository,
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
    InstrumentedAnlageRepository,
    AnlageRepository,
    anlagen,
    methods: [
        #[instrument(skip(self, anlage), level = "debug", err(Debug))]
        fn create(&self, anlage: NeueAnlage) -> RepositoryResult<Versioned<Anlage>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: AnlageId) -> RepositoryResult<Versioned<Anlage>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_aktiv(&self) -> RepositoryResult<Vec<Versioned<Anlage>>>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "trace", err(Debug))]
        fn find_by_verwaltung(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Versioned<Anlage>>>;
        #[instrument(skip(self, anlage), level = "debug", err(Debug))]
        fn update(&self, anlage: &mut Versioned<Anlage>) -> RepositoryResult<()>;
        #[instrument(skip(self, anlage), level = "debug", err(Debug))]
        fn delete(&self, anlage: Versioned<Anlage>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedRundeRepository,
    RundeRepository,
    runden,
    methods: [
        #[instrument(skip(self, runde), level = "debug", err(Debug))]
        fn create(&self, runde: NeueRunde) -> RepositoryResult<Versioned<Runde>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: RundeId) -> RepositoryResult<Versioned<Runde>>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "trace", err(Debug))]
        fn find_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Versioned<Runde>>>;
        #[instrument(skip(self, runde), level = "debug", err(Debug))]
        fn update(&self, runde: &mut Versioned<Runde>) -> RepositoryResult<()>;
        #[instrument(skip(self, runde), level = "debug", err(Debug))]
        fn delete(&self, runde: Versioned<Runde>) -> RepositoryResult<()>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn exists_referencing_anlage(&self, id: AnlageId) -> RepositoryResult<bool>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn exists_referencing_parcours(&self, id: ParcoursId) -> RepositoryResult<bool>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn exists_referencing_ziel(&self, id: ZielId) -> RepositoryResult<bool>;
    ]
);

impl_instrumented_repo!(
    InstrumentedSchussRepository,
    SchussRepository,
    schüsse,
    methods: [
        #[instrument(skip(self, schuss), level = "debug", err(Debug))]
        fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Versioned<Schuss>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: SchussId) -> RepositoryResult<Versioned<Schuss>>;
        #[instrument(skip(self, runde_id), fields(runde_id = ?runde_id), level = "trace", err(Debug))]
        fn find_by_runde(&self, runde_id: RundeId) -> RepositoryResult<Vec<Versioned<Schuss>>>;
        #[instrument(skip(self, ziel_id), fields(ziel_id = ?ziel_id), level = "trace", err(Debug))]
        fn find_by_ziel(&self, ziel_id: ZielId) -> RepositoryResult<Vec<Versioned<Schuss>>>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "trace", err(Debug))]
        fn find_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Versioned<Schuss>>>;
        #[instrument(skip(self, schuss), level = "debug", err(Debug))]
        fn update(&self, schuss: &mut Versioned<Schuss>) -> RepositoryResult<()>;
        #[instrument(skip(self, schuss), level = "debug", err(Debug))]
        fn delete(&self, schuss: Versioned<Schuss>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedBenutzerRepository,
    BenutzerRepository,
    benutzer,
    methods: [
        #[instrument(skip(self, benutzer), level = "debug", err(Debug))]
        fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Versioned<Benutzer>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Versioned<Benutzer>>;
        #[instrument(skip(self, benutzer), level = "debug", err(Debug))]
        fn update(&self, benutzer: &mut Versioned<Benutzer>) -> RepositoryResult<()>;
        #[instrument(skip(self, benutzer), level = "debug", err(Debug))]
        fn delete(&self, benutzer: Versioned<Benutzer>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedKontoRepository,
    KontoRepository,
    konten,
    methods: [
        #[instrument(skip(self, konto), level = "debug", err(Debug))]
        fn create(&self, konto: NeuesKonto) -> RepositoryResult<Versioned<Konto>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: KontoId) -> RepositoryResult<Versioned<Konto>>;
        #[instrument(skip(self, email), fields(email = email.as_ref()), level = "trace", err(Debug))]
        fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Versioned<Konto>>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "trace", err(Debug))]
        fn find_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Versioned<Konto>>>;
        #[instrument(skip(self, konto), level = "debug", err(Debug))]
        fn update(&self, konto: &mut Versioned<Konto>) -> RepositoryResult<()>;
        #[instrument(skip(self, konto), level = "debug", err(Debug))]
        fn delete(&self, konto: Versioned<Konto>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedSessionRepository,
    SessionRepository,
    sessions,
    methods: [
        #[instrument(skip(self, session), level = "debug", err(Debug))]
        fn create(&self, session: NeueSession) -> RepositoryResult<Versioned<Session>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: SessionId) -> RepositoryResult<Versioned<Session>>;
        #[instrument(skip(self, hash), level = "trace", err(Debug))]
        fn find_by_refresh_hash(&self, hash: &SecretTokenHash) -> RepositoryResult<Versioned<Session>>;
        #[instrument(skip(self, session), level = "debug", err(Debug))]
        fn update(&self, session: &mut Versioned<Session>) -> RepositoryResult<()>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "trace", err(Debug))]
        fn find_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Versioned<Session>>>;
        #[instrument(skip(self, session), level = "debug", err(Debug))]
        fn delete(&self, session: Versioned<Session>) -> RepositoryResult<()>;
        #[instrument(skip(self, benutzer_id), fields(benutzer_id = ?benutzer_id), level = "debug", err(Debug))]
        fn delete_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedEmailBestätigungRepository,
    EmailBestätigungRepository,
    bestätigungen,
    methods: [
        #[instrument(skip(self, bestätigung), level = "debug", err(Debug))]
        fn create(&self, bestätigung: NeueBestätigung) -> RepositoryResult<Versioned<EmailBestätigung>>;
        #[instrument(skip(self, hash), level = "trace", err(Debug))]
        fn find_by_hash(&self, hash: &SecretTokenHash) -> RepositoryResult<Versioned<EmailBestätigung>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: BestätigungId) -> RepositoryResult<Versioned<EmailBestätigung>>;
        #[instrument(skip(self, email), fields(email = email.as_ref()), level = "trace", err(Debug))]
        fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Vec<Versioned<EmailBestätigung>>>;
        #[instrument(skip(self, bestätigung), level = "debug", err(Debug))]
        fn update(&self, bestätigung: &mut Versioned<EmailBestätigung>) -> RepositoryResult<()>;
        #[instrument(skip(self, bestätigung), level = "debug", err(Debug))]
        fn delete(&self, bestätigung: Versioned<EmailBestätigung>) -> RepositoryResult<()>;
        #[instrument(skip(self, email), fields(email = email.as_ref()), level = "debug", err(Debug))]
        fn delete_by_email(&self, email: &EmailAdresse) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedPasswortWiederherstellungRepository,
    PasswortWiederherstellungRepository,
    wiederherstellungen,
    methods: [
        #[instrument(skip(self, wiederherstellung), level = "debug", err(Debug))]
        fn create(&self, wiederherstellung: NeuePasswortWiederherstellung) -> RepositoryResult<Versioned<PasswortWiederherstellung>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: PasswortWiederherstellungId) -> RepositoryResult<Versioned<PasswortWiederherstellung>>;
        #[instrument(skip(self, hash), level = "trace", err(Debug))]
        fn find_by_hash(&self, hash: &SecretTokenHash) -> RepositoryResult<Versioned<PasswortWiederherstellung>>;
        #[instrument(skip(self, konto_id), fields(konto_id = ?konto_id), level = "trace", err(Debug))]
        fn find_by_konto(&self, konto_id: KontoId) -> RepositoryResult<Vec<Versioned<PasswortWiederherstellung>>>;
        #[instrument(skip(self, wiederherstellung), level = "debug", err(Debug))]
        fn update(&self, wiederherstellung: &mut Versioned<PasswortWiederherstellung>) -> RepositoryResult<()>;
        #[instrument(skip(self, wiederherstellung), level = "debug", err(Debug))]
        fn delete(&self, wiederherstellung: Versioned<PasswortWiederherstellung>) -> RepositoryResult<()>;
        #[instrument(skip(self, konto_id), fields(konto_id = ?konto_id), level = "debug", err(Debug))]
        fn delete_by_konto(&self, konto_id: KontoId) -> RepositoryResult<()>;
    ]
);

/// Pinned bundle: instrumented repos hold pointers into `uow` (field order keeps drop safe).
pub(crate) struct RepoStorage {
    pub(crate) anlagen: InstrumentedAnlageRepository,
    pub(crate) runden: InstrumentedRundeRepository,
    pub(crate) schüsse: InstrumentedSchussRepository,
    pub(crate) benutzer: InstrumentedBenutzerRepository,
    pub(crate) konten: InstrumentedKontoRepository,
    pub(crate) sessions: InstrumentedSessionRepository,
    pub(crate) bestätigungen: InstrumentedEmailBestätigungRepository,
    pub(crate) wiederherstellungen: InstrumentedPasswortWiederherstellungRepository,
    pub(crate) uow: Box<dyn UnitOfWorkImpl>,
}

impl RepoStorage {
    pub(crate) fn new(uow: Box<dyn UnitOfWorkImpl>) -> Pin<Box<Self>> {
        let mut pinned = Box::pin(RepoStorage {
            anlagen: InstrumentedAnlageRepository::disabled(),
            runden: InstrumentedRundeRepository::disabled(),
            schüsse: InstrumentedSchussRepository::disabled(),
            benutzer: InstrumentedBenutzerRepository::disabled(),
            konten: InstrumentedKontoRepository::disabled(),
            sessions: InstrumentedSessionRepository::disabled(),
            bestätigungen: InstrumentedEmailBestätigungRepository::disabled(),
            wiederherstellungen: InstrumentedPasswortWiederherstellungRepository::disabled(),
            uow,
        });

        // SAFETY: `pinned` is boxed, so `uow`'s address is stable for the lifetime of the
        // storage. The wrappers are never moved out; they are dropped in place with it.
        unsafe {
            let this = pinned.as_mut().get_unchecked_mut();
            let uow_ptr = std::ptr::from_ref(this.uow.as_ref());
            this.anlagen = InstrumentedAnlageRepository::new(uow_ptr);
            this.runden = InstrumentedRundeRepository::new(uow_ptr);
            this.schüsse = InstrumentedSchussRepository::new(uow_ptr);
            this.benutzer = InstrumentedBenutzerRepository::new(uow_ptr);
            this.konten = InstrumentedKontoRepository::new(uow_ptr);
            this.sessions = InstrumentedSessionRepository::new(uow_ptr);
            this.bestätigungen = InstrumentedEmailBestätigungRepository::new(uow_ptr);
            this.wiederherstellungen =
                InstrumentedPasswortWiederherstellungRepository::new(uow_ptr);
        }

        pinned
    }
}
