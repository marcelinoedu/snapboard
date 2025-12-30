use std::fs;

use crate::state::database_state::DbState;
use crate::db::utils::now_ts;
use sqlx::Row;


pub async fn cleanup_expired(db: &DbState) {
    let now = now_ts();

    let rows = sqlx::query(
        "SELECT file_path FROM clipboard_items WHERE expires_at <= ?"
    )
    .bind(now)
    .fetch_all(&db.pool)
    .await
    .unwrap_or_default();

    for row in rows {
        if let Ok(Some(path)) = row.try_get::<Option<String>, _>("file_path") {
            let _ = fs::remove_file(path);
        }
    }

    let _ = sqlx::query(
        "DELETE FROM clipboard_items WHERE expires_at <= ?"
    )
    .bind(now)
    .execute(&db.pool)
    .await;
}
