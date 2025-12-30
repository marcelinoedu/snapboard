use crate::models::clipboard_item::ClipboardItem;
use crate::utils::hash_content;

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use std::sync::RwLock;

#[derive(Clone)]
pub struct ClipboardState {
    current_clip: Arc<RwLock<Option<ClipboardItem>>>,
    clip_history: Arc<RwLock<VecDeque<ClipboardItem>>>,
    contents: Arc<RwLock<HashSet<u64>>>,
    max_items: usize,
}

impl ClipboardState {

    const MAX_ITEMS_DEFAULT: usize = 100;

    pub fn new() -> Self {
        ClipboardState {
            current_clip: Arc::new(RwLock::new(None)),
            clip_history: Arc::new(RwLock::new(VecDeque::new())),
            contents: Arc::new(RwLock::new(HashSet::new())),
            max_items: Self::MAX_ITEMS_DEFAULT,
        }
    }

    pub fn push_item(&self, clip_item: ClipboardItem) -> bool {
        *self.current_clip.write().unwrap() = Some(clip_item.clone());

        let mut contents = self.contents.write().unwrap();
        if !contents.insert(hash_content(&clip_item.content)) {
            return false;
        }

        let mut history = self.clip_history.write().unwrap();
        history.push_back(clip_item);

        if history.len() > self.max_items {
            if let Some(removed) = history.pop_front() {
                contents.remove(&hash_content(&removed.content));
            }
        }

        true
    }

    pub fn remove_item(&self, item_id: &str) -> bool {
        let mut contents = self.contents.write().unwrap();
        let mut history = self.clip_history.write().unwrap();

        if let Some(pos) = history.iter().position(|i| i.id == item_id) {
            let removed = history.remove(pos).unwrap();
            contents.remove(&hash_content(&removed.content));
            return true;
        }

        false
    }

    pub fn list_clipboard_history(&self) -> Vec<ClipboardItem> {
        self.clip_history
            .read()
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }

    pub fn get_current_clip(&self) -> Option<ClipboardItem> {
        self.current_clip.read().unwrap().clone()
    }

    pub fn delete_all(&self) {
        self.clip_history.write().unwrap().clear();
        self.contents.write().unwrap().clear();
    }
}
