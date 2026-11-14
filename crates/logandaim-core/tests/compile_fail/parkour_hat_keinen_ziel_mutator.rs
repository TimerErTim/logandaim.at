//! A `RundeIn<MitParkourInBearbeitung>` has no route mutator: its route is captured, never edited.
//!
//! Each of these three calls must fail to resolve, which is what makes AC-22's "a Parkour
//! round's target list is frozen" a compile-time guarantee rather than a runtime refusal.

use chrono::{DateTime, Utc};
use logandaim_core::domain::{
    AnlageId, Erfasser, NonEmptyString, ParkourId, Runde, RundeId, RundenHerkunft, Rundenaufbau,
    ZielId, konto::BenutzerId,
};
use uuid::Uuid;

fn parkour_runde() -> Runde {
    Runde::from_parts(
        RundeId(Uuid::nil()),
        Erfasser::Benutzer(BenutzerId(Uuid::nil())),
        NonEmptyString::neu("Waldrunde").unwrap(),
        Some(RundenHerkunft {
            anlage_id: AnlageId(Uuid::nil()),
            parkour_id: ParkourId(Uuid::nil()),
        }),
        Rundenaufbau::from_parts(Vec::new()),
        DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
        None,
    )
}

fn main() {
    let Runde::MitParkour(mut runde) = parkour_runde() else {
        return;
    };
    runde.ziel_hinzufügen(None);
    runde.ziel_entfernen(ZielId(Uuid::nil()));
    runde.ziel_umbenennen(ZielId(Uuid::nil()), NonEmptyString::neu("Neu").unwrap());
}
