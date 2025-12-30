use crate::models::clipboard_item::ClipboardItem;
use crate::state::database_state::DbState;
use sqlx::{Row, SqlitePool};

#[derive(Clone)]
pub struct ClipboardItemRepository {
    pool: SqlitePool,
}

impl ClipboardItemRepository {
    pub fn new(db: &DbState) -> Self {
        Self {
            pool: db.pool.clone(),
        }
    }

    pub async fn list_items(&self) -> Result<Vec<ClipboardItem>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, kind, content, file_path, created_at, expires_at
            FROM clipboard_items
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ClipboardItem {
                id: row.get("id"),
                kind: row.get("kind"),
                content: row.get("content"),
                file_path: row.get("file_path"),
                created_at: row.get("created_at"),
                expires_at: row.get("expires_at"),
            })
            .collect())
    }

    pub async fn delete_item(&self, item_id: String) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM clipboard_items WHERE id = ?")
            .bind(item_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn create_item(&self, item: ClipboardItem) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO clipboard_items (
                id, kind, content, file_path, created_at, expires_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(item.id)
        .bind(item.kind)
        .bind(item.content)
        .bind(item.file_path)
        .bind(item.created_at)
        .bind(item.expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
