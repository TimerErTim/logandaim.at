//! A closed round has no `pruefe_position`: the typestate makes "validate a shot against a
//! route" unreachable once the round is closed, so the call must fail to resolve.

use chrono::{DateTime, Utc};
use logandaim_core::domain::{
    Erfasser, NonEmptyString, Runde, RundeId, Rundenaufbau, ZielId, konto::BenutzerId,
};
use uuid::Uuid;

fn abgeschlossene_runde() -> Runde {
    Runde::from_parts(
        RundeId(Uuid::nil()),
        Erfasser::Benutzer(BenutzerId(Uuid::nil())),
        NonEmptyString::neu("Waldrunde").unwrap(),
        None,
        Rundenaufbau::from_parts(Vec::new()),
        DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
        Some(DateTime::<Utc>::from_timestamp(1, 0).unwrap()),
    )
}

fn main() {
    let Runde::AbgeschlosseneRunde(runde) = abgeschlossene_runde() else {
        return;
    };
    let _ = runde.pruefe_position(ZielId(Uuid::nil()), None);
}
