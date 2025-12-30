use std::fs;
use std::path::PathBuf;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tauri::App;

use crate::state::database_state::DbState;

pub async fn init_db(_app: &App) -> Result<DbState, Box<dyn std::error::Error>> {
    let base: PathBuf = dirs::home_dir()
        .expect("no home dir")
        .join("Library")
        .join("Application Support")
        .join("SnapBoardDev");

    fs::create_dir_all(&base)?;
    let db_path = base.join("snapboard.db");

    println!("SQLite DB path: {}", db_path.display());

    
    let connect_options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(DbState { pool })
}
