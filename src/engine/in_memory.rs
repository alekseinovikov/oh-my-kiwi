use crate::core::Engine;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub(crate) struct InMemoryEngine {
    storage: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
}

impl InMemoryEngine {
    pub fn new() -> Self {
        Self {
            storage: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Engine for InMemoryEngine {
    async fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
        let storage = self.storage.read().await;
        storage.get(key).cloned()
    }

    async fn set(&self, key: Vec<u8>, value: Vec<u8>) {
        let mut storage = self.storage.write().await;
        storage.insert(key, value);
    }
}
