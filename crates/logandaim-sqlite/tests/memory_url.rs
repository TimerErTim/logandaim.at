//! Regression: `:memory:` must be one database for the whole instance.
//!
//! `libsql::Builder::new_local(":memory:")` gives every connection its own private
//! database. Migrations run on one connection and queries later run on another, so the
//! schema appeared to vanish and the server died with `no such table: sessions` at
//! startup. The URL has to route through the shared-cache in-memory instance instead.

use logandaim_sqlite::SQLiteInstance;

#[test_log::test(pollster::test)]
async fn memory_url_shares_one_database_across_connections() {
    let instance = SQLiteInstance::local(":memory:").await.unwrap();
    instance.migrate_repos_to_latest().await.unwrap();

    // The migration ran on its own connection; a fresh one must see its tables.
    let connection = instance.create_connection().await.unwrap();
    let mut rows = connection
        .query(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'sessions'",
            (),
        )
        .await
        .unwrap();

    assert!(
        rows.next().await.unwrap().is_some(),
        ":memory: connections do not share the migrated schema"
    );
}
