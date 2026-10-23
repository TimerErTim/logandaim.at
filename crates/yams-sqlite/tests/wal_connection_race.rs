//! Regression: parallel `execute_fn` must not fail opening connections with WAL
//! while another UoW is committing/closing (checkpoint / vacuum lock window).

use std::sync::Arc;

use yams_core::App;
use yams_core::ports::RepositoryError;
use yams_sqlite::SQLiteInstance;

async fn app() -> Arc<App> {
    let mut sqlite = SQLiteInstance::in_temp_dir().await.unwrap();
    sqlite.migrate_repos_to_latest().await.unwrap();
    Arc::new(App::builder().uow_provider(Box::new(sqlite)).build())
}

#[test_log::test(pollster::test)]
async fn parallel_execute_fn_survives_wal_connection_init() {
    let app = app().await;

    // Pair a writer (commit + connection drop/checkpoint) with an opener that
    // immediately begins a new UoW. Pre-fix, each open re-ran
    // `PRAGMA journal_mode=WAL` and flaked with "database is locked".
    // Keep the loop modest: enough overlap to stress locks, not dominate CI time.
    let mut handles = Vec::with_capacity(128);
    for _ in 0..64u64 {
        let writer_app = Arc::clone(&app);
        let opener_app = Arc::clone(&app);
        handles.push(std::thread::spawn(move || {
            pollster::block_on(async move {
                writer_app
                    .execute_fn::<_, _, RepositoryError>(async |ctx| {
                        let uow = ctx.enter().await?;
                        uow.commit().await
                    })
                    .await
                    .expect("writer execute_fn must not fail with database locked");
            })
        }));
        handles.push(std::thread::spawn(move || {
            pollster::block_on(async move {
                opener_app
                    .execute_fn::<_, _, RepositoryError>(async |ctx| {
                        let uow = ctx.enter().await?;
                        uow.rollback().await
                    })
                    .await
                    .expect("opener execute_fn must not fail with database locked / WAL init");
            })
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
}
