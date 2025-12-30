CREATE TABLE IF NOT EXISTS clipboard_items (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  content TEXT,
  file_path TEXT,
  created_at INTEGER NOT NULL,
  expires_at INTEGER NOT NULL
);

CREATE INDEX idx_items_expires_at ON clipboard_items(expires_at);
