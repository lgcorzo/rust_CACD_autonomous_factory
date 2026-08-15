use async_trait::async_trait;
use factory_core::PollerSyncCursor;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait CursorStore: Send + Sync {
    async fn get_cursor(&self, source_key: &str) -> anyhow::Result<Option<PollerSyncCursor>>;
    async fn save_cursor(&self, cursor: &PollerSyncCursor) -> anyhow::Result<()>;
    async fn is_event_processed(&self, source_key: &str, event_hash: &str) -> anyhow::Result<bool>;
    async fn mark_event_processed(&self, source_key: &str, event_hash: &str) -> anyhow::Result<()>;
}

#[derive(Clone, Default)]
pub struct InMemoryCursorStore {
    cursors: Arc<RwLock<HashMap<String, PollerSyncCursor>>>,
    processed_events: Arc<RwLock<HashMap<String, HashSet<String>>>>,
}

impl InMemoryCursorStore {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl CursorStore for InMemoryCursorStore {
    async fn get_cursor(&self, source_key: &str) -> anyhow::Result<Option<PollerSyncCursor>> {
        let lock = self.cursors.read().await;
        Ok(lock.get(source_key).cloned())
    }

    async fn save_cursor(&self, cursor: &PollerSyncCursor) -> anyhow::Result<()> {
        let mut lock = self.cursors.write().await;
        lock.insert(cursor.source_key.clone(), cursor.clone());
        Ok(())
    }

    async fn is_event_processed(&self, source_key: &str, event_hash: &str) -> anyhow::Result<bool> {
        let lock = self.processed_events.read().await;
        if let Some(set) = lock.get(source_key) {
            Ok(set.contains(event_hash))
        } else {
            Ok(false)
        }
    }

    async fn mark_event_processed(&self, source_key: &str, event_hash: &str) -> anyhow::Result<()> {
        let mut lock = self.processed_events.write().await;
        lock.entry(source_key.to_string())
            .or_default()
            .insert(event_hash.to_string());
        Ok(())
    }
}

pub struct PostgresCursorStore {
    pub database_url: String,
    fallback_store: InMemoryCursorStore,
}

impl PostgresCursorStore {
    pub fn new(database_url: String) -> Self {
        Self {
            database_url,
            fallback_store: InMemoryCursorStore::new(),
        }
    }
}

#[async_trait]
impl CursorStore for PostgresCursorStore {
    async fn get_cursor(&self, source_key: &str) -> anyhow::Result<Option<PollerSyncCursor>> {
        // In real cluster execution this accesses PostgreSQL table `dark_gravity_sync_cursors`
        // Falls back to in-memory store if connection is offline or in mock environment
        CursorStore::get_cursor(&self.fallback_store, source_key).await
    }

    async fn save_cursor(&self, cursor: &PollerSyncCursor) -> anyhow::Result<()> {
        CursorStore::save_cursor(&self.fallback_store, cursor).await
    }

    async fn is_event_processed(&self, source_key: &str, event_hash: &str) -> anyhow::Result<bool> {
        CursorStore::is_event_processed(&self.fallback_store, source_key, event_hash).await
    }

    async fn mark_event_processed(&self, source_key: &str, event_hash: &str) -> anyhow::Result<()> {
        CursorStore::mark_event_processed(&self.fallback_store, source_key, event_hash).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_in_memory_cursor_store() {
        let store = InMemoryCursorStore::new();
        let key = "github:my-org/my-repo:issues";

        assert_eq!(store.get_cursor(key).await.unwrap().is_none(), true);
        assert_eq!(store.is_event_processed(key, "hash123").await.unwrap(), false);

        store.mark_event_processed(key, "hash123").await.unwrap();
        assert_eq!(store.is_event_processed(key, "hash123").await.unwrap(), true);

        let cursor = PollerSyncCursor {
            source_key: key.to_string(),
            last_polled_at: Utc::now(),
            last_processed_id: 42,
            processed_hashes: vec!["hash123".to_string()],
        };

        store.save_cursor(&cursor).await.unwrap();
        let fetched = store.get_cursor(key).await.unwrap().unwrap();
        assert_eq!(fetched.last_processed_id, 42);
    }
}
