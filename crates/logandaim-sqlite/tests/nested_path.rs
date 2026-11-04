//! Regression: a database path may name a directory that does not exist yet.
//!
//! libsql creates the database file but not the directory holding it, so a path such as
//! `data.local/logandaim.db` failed with `Unable to open connection ... (14)` on a first
//! run. The default configuration uses exactly such a path so that throwaway state stays
//! inside one ignored directory, which made a bare `logandaim-server` start fail.

use logandaim_sqlite::SQLiteInstance;

#[test_log::test(pollster::test)]
async fn a_missing_parent_directory_is_created() {
    let root = tempdir::TempDir::new("logandaim-nested-path").unwrap();
    let nested = root.path().join("data.local").join("logandaim.db");
    assert!(
        !nested.parent().unwrap().exists(),
        "the test needs a parent that does not exist yet"
    );

    let instance = SQLiteInstance::local(&nested).await.expect("open database");
    instance.migrate_repos_to_latest().await.unwrap();
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
        "the database opened through a missing parent never migrated"
    );
}
