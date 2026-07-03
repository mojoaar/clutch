#[tokio::test]
async fn init_pool_creates_database_and_runs_migrations() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("clutch.db");

    let pool = app_lib::db::init_pool(db_path.clone()).await.unwrap();

    // Verify the file exists on disk
    assert!(db_path.exists());

    // Verify we can query a table from migrations
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sessions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 0);
}

#[tokio::test]
async fn init_pool_automatically_creates_parent_directories() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("nested").join("deeply").join("clutch.db");

    let pool = app_lib::db::init_pool(db_path.clone()).await.unwrap();

    assert!(db_path.exists());

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM settings")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(count.0 > 0); // Contains migration inserts
}
