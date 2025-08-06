use anyhow::Context;
use sqlx::sqlite::Sqlite;
use sqlx::Pool;
use std::env;
use std::fs;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Result;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self> {
        let app_dir = app_handle.path().app_data_dir()?;

        fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("app.db");

        // Set the DATABASE_URL environment variable to point to this SQLite file
        env::set_var("DATABASE_URL", format!("sqlite://{}", db_path.display()));

        let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = Pool::<Sqlite>::connect_with(connection_options)
            .await
            .context("Failed to connect to the database")?;

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("Failed to run db migrations")?;

        Ok(Self { pool })
    }
}
