use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum ClipboardKind {
    text,
    link,
    code,
    image,
    file,
}

#[derive(Clone, Serialize)]
pub struct ClipboardItem {
    pub id: String,
    pub kind: ClipboardKind,
    pub content: String,
    pub file_path: Option<String>,
    pub created_at: i64,
    pub expires_at: i64,
}
