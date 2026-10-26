//! Client-scoped throttle in one process's memory.
//!
//! This is the **production** adapter, not a test double: logandaim.at runs as one process
//! against one SQLite file, so a shared store would add a network hop and a second state to
//! keep consistent for no gain. A multi-process deployment would need a different adapter,
//! and that would be a deliberate change, not a default.
//!
//! Expired windows reset on read, so nothing here needs a background sweeper. The downside
//! is honest and worth stating: a restart forgives every spent budget, and memory grows with
//! the number of distinct clients. Both are acceptable at this size and neither weakens the
//! threshold behaviour AC-15 and AC-16 assert.

use std::collections::HashMap;
use std::sync::Mutex;

use chrono::{DateTime, Utc};

use crate::{
    ResultReport,
    ports::{ClientKey, Throttle, ThrottleAction, ThrottleError},
};

/// One client's budget for one action.
struct Window {
    start: DateTime<Utc>,
    count: u32,
}

pub struct InMemoryThrottle {
    windows: Mutex<HashMap<(ClientKey, &'static str), Window>>,
}

impl InMemoryThrottle {
    pub fn new() -> Self {
        Self {
            windows: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryThrottle {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Throttle for InMemoryThrottle {
    async fn check_and_count(
        &self,
        client: &ClientKey,
        action: ThrottleAction,
        now: DateTime<Utc>,
    ) -> ResultReport<(), ThrottleError> {
        let mut windows = self
            .windows
            .lock()
            .expect("the throttle mutex is not poisoned");
        let entry = windows
            .entry((client.clone(), action.as_str()))
            .or_insert_with(|| Window {
                start: now,
                count: 0,
            });
        if now - entry.start >= action.window() {
            entry.start = now;
            entry.count = 0;
        }
        if entry.count >= action.threshold() {
            return Err(error_stack::Report::new(ThrottleError::TooManyAttempts));
        }
        entry.count += 1;
        Ok(())
    }

    async fn reset(
        &self,
        client: &ClientKey,
        action: ThrottleAction,
    ) -> ResultReport<(), ThrottleError> {
        self.windows
            .lock()
            .expect("the throttle mutex is not poisoned")
            .remove(&(client.clone(), action.as_str()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::*;

    fn client(fingerprint: &str) -> ClientKey {
        ClientKey::new(
            "203.0.113.7".parse::<IpAddr>().expect("static address"),
            fingerprint,
        )
    }

    fn now() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).expect("fixed instant")
    }

    #[test_log::test(pollster::test)]
    async fn das_budget_wird_genau_einmal_ausgeschöpft() {
        let throttle = InMemoryThrottle::new();
        let client = client("a");
        let action = ThrottleAction::RegistrierungStart;

        for i in 0..action.threshold() {
            assert!(
                throttle
                    .check_and_count(&client, action, now())
                    .await
                    .is_ok(),
                "Versuch {i} liegt im Budget"
            );
        }
        assert!(
            throttle
                .check_and_count(&client, action, now())
                .await
                .is_err()
        );
    }

    #[test_log::test(pollster::test)]
    async fn nach_dem_fenster_ist_das_budget_wieder_frei() {
        let throttle = InMemoryThrottle::new();
        let client = client("a");
        let action = ThrottleAction::RegistrierungStart;

        for _ in 0..action.threshold() {
            throttle
                .check_and_count(&client, action, now())
                .await
                .unwrap();
        }
        assert!(
            throttle
                .check_and_count(&client, action, now())
                .await
                .is_err()
        );

        let later = now() + action.window();
        assert!(
            throttle
                .check_and_count(&client, action, later)
                .await
                .is_ok()
        );
    }

    #[test_log::test(pollster::test)]
    async fn aktionen_und_clients_haben_getrennte_budgets() {
        let throttle = InMemoryThrottle::new();
        let a = client("a");
        let b = client("b");

        for _ in 0..ThrottleAction::RegistrierungStart.threshold() {
            throttle
                .check_and_count(&a, ThrottleAction::RegistrierungStart, now())
                .await
                .unwrap();
        }
        assert!(
            throttle
                .check_and_count(&a, ThrottleAction::RegistrierungStart, now())
                .await
                .is_err()
        );

        // Another client, and another action of the same client, are unaffected.
        assert!(
            throttle
                .check_and_count(&b, ThrottleAction::RegistrierungStart, now())
                .await
                .is_ok()
        );
        assert!(
            throttle
                .check_and_count(&a, ThrottleAction::Anmeldung, now())
                .await
                .is_ok()
        );
    }

    #[test_log::test(pollster::test)]
    async fn zurücksetzen_gibt_das_budget_frei() {
        let throttle = InMemoryThrottle::new();
        let client = client("a");
        let action = ThrottleAction::Anmeldung;

        for _ in 0..action.threshold() {
            throttle
                .check_and_count(&client, action, now())
                .await
                .unwrap();
        }
        assert!(
            throttle
                .check_and_count(&client, action, now())
                .await
                .is_err()
        );

        throttle.reset(&client, action).await.unwrap();
        assert!(
            throttle
                .check_and_count(&client, action, now())
                .await
                .is_ok()
        );
    }
}
