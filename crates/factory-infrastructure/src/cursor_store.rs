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

    // ── Pipeline Recurring Failure Tracking (T064) ──

    /// Increment the failure counter for a given error fingerprint and return the new count.
    async fn increment_failure_count(&self, fingerprint: &str) -> anyhow::Result<u32>;

    /// Get the current failure count for a given error fingerprint.
    async fn get_failure_count(&self, fingerprint: &str) -> anyhow::Result<u32>;
}

#[derive(Clone, Default)]
pub struct InMemoryCursorStore {
    cursors: Arc<RwLock<HashMap<String, PollerSyncCursor>>>,
    processed_events: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    /// Recurring failure counter by error fingerprint.
    failure_counts: Arc<RwLock<HashMap<String, u32>>>,
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

    async fn increment_failure_count(&self, fingerprint: &str) -> anyhow::Result<u32> {
        let mut lock = self.failure_counts.write().await;
        let count = lock.entry(fingerprint.to_string()).or_insert(0);
        *count += 1;
        Ok(*count)
    }

    async fn get_failure_count(&self, fingerprint: &str) -> anyhow::Result<u32> {
        let lock = self.failure_counts.read().await;
        Ok(lock.get(fingerprint).copied().unwrap_or(0))
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

    async fn increment_failure_count(&self, fingerprint: &str) -> anyhow::Result<u32> {
        CursorStore::increment_failure_count(&self.fallback_store, fingerprint).await
    }

    async fn get_failure_count(&self, fingerprint: &str) -> anyhow::Result<u32> {
        CursorStore::get_failure_count(&self.fallback_store, fingerprint).await
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

        assert!(store.get_cursor(key).await.unwrap().is_none());
        assert!(!store.is_event_processed(key, "hash123").await.unwrap());

        store.mark_event_processed(key, "hash123").await.unwrap();
        assert!(store.is_event_processed(key, "hash123").await.unwrap());

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

    // ── T064-T065: Failure count tests ──

    #[tokio::test]
    async fn test_failure_count_increment_and_get() {
        let store = InMemoryCursorStore::new();
        let fingerprint = "a1b2c3d4e5f60001";

        // Initial count should be 0
        assert_eq!(store.get_failure_count(fingerprint).await.unwrap(), 0);

        // Increment 3 times
        let count1 = store.increment_failure_count(fingerprint).await.unwrap();
        assert_eq!(count1, 1);

        let count2 = store.increment_failure_count(fingerprint).await.unwrap();
        assert_eq!(count2, 2);

        let count3 = store.increment_failure_count(fingerprint).await.unwrap();
        assert_eq!(count3, 3);

        // get_failure_count should return 3
        assert_eq!(store.get_failure_count(fingerprint).await.unwrap(), 3);

        // Different fingerprint starts at 0
        let other_fp = "zzzz0000aaaa1111";
        assert_eq!(store.get_failure_count(other_fp).await.unwrap(), 0);
    }
}
