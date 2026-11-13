//! Inbound requests of a Runde.
//!
//! The owner is never part of a body: it comes from the verified access token, and the use
//! case decides authorisation from the aggregate it loads (SPEC-konto rule 17).
//!
//! Two bodies start a Runde, because a `Runde` has exactly one setup: one names the Parkour,
//! the other names only itself. A body that named both would be a `Rundenquelle` two variants
//! deep, not a request.

use uuid::Uuid;

use error_stack::{Report, ResultExt};

use logandaim_core::{
    domain::{NonEmptyString, ParkourId, RundeId, ZielId},
    service::{
        RundeStarten, RundeStartenFehler, RundeZielEntfernen, RundeZielHinzufügen,
        RundeZielHinzufügenFehler, RundeZielUmbenennen, RundeZielUmbenennenFehler,
    },
};

/// Start a Runde on one active Parkour.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeMitParkourStartenRequest {
    /// The Parkour to shoot. It must be active.
    pub parkour_id: Uuid,
}

impl From<RundeMitParkourStartenRequest> for RundeStarten {
    fn from(anfrage: RundeMitParkourStartenRequest) -> Self {
        Self::mit_parkour(ParkourId(anfrage.parkour_id))
    }
}

/// Start a `Freie Runde`: no Anlage, no Parkour, and its own name.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreieRundeStartenRequest {
    /// What the archer calls this round; it is the round's own name, not a Parkour's.
    pub name: String,
}

impl TryFrom<FreieRundeStartenRequest> for RundeStarten {
    type Error = Report<RundeStartenFehler>;

    /// Validate at the boundary: an empty name is not a name, so the domain never sees one.
    fn try_from(anfrage: FreieRundeStartenRequest) -> Result<Self, Self::Error> {
        NonEmptyString::neu(anfrage.name.trim())
            .change_context(RundeStartenFehler::UngültigeAnfrage)
            .attach("Freie Runde: Name")
            .map(RundeStarten::ohne_parkour)
    }
}

/// Append one `Freies Ziel` to a free Runde.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZielHinzufuegenRequest {
    /// The Ziel's name; absent means "Ziel N" from its ordinal.
    pub name: Option<String>,
}

impl ZielHinzufuegenRequest {
    /// Present-but-blank is a refusal, absent is the auto-name case.
    ///
    /// `{"name": "   "}` used to reach `None` and be answered 200 with a generated name,
    /// while the same body on the rename route is a 400. The two routes now agree: only an
    /// absent name asks the round to number the Ziel itself.
    pub fn in_use_case(
        self,
        runde_id: RundeId,
    ) -> Result<RundeZielHinzufügen, Report<RundeZielHinzufügenFehler>> {
        let name = match self.name {
            None => None,
            Some(name) => Some(
                NonEmptyString::neu(name.trim())
                    .change_context(RundeZielHinzufügenFehler::UngültigeAnfrage)
                    .attach("Freies Ziel: Name")?,
            ),
        };
        Ok(RundeZielHinzufügen { runde_id, name })
    }
}

/// Rename one free Ziel inside its Runde.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZielUmbenennenRequest {
    pub name: String,
}

impl ZielUmbenennenRequest {
    pub fn in_use_case(
        self,
        runde_id: RundeId,
        ziel_id: Uuid,
    ) -> Result<RundeZielUmbenennen, Report<RundeZielUmbenennenFehler>> {
        Ok(RundeZielUmbenennen {
            runde_id,
            ziel_id: ZielId(ziel_id),
            name: NonEmptyString::neu(self.name.trim())
                .change_context(RundeZielUmbenennenFehler::UngültigeAnfrage)
                .attach("Freies Ziel: Name")?,
        })
    }
}

/// The path of one free Ziel inside its Runde.
pub fn ziel_entfernen_in_use_case(runde_id: RundeId, ziel_id: ZielId) -> RundeZielEntfernen {
    RundeZielEntfernen { runde_id, ziel_id }
}
