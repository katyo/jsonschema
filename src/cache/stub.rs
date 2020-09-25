use crate::Args;

/// Cache store
pub struct Cache;

impl Cache {
    pub fn open(_args: &Args, _group: impl AsRef<str>) -> Self {
        Self
    }

    /// Get data from cache
    pub fn get<K, V>(&self, _key: &K) -> Option<V> {
        None
    }

    /// Store data into cache
    pub fn put<K, V>(&self, _key: &K, _val: &V) {}

    /// Remove data from cache
    pub fn del<K>(&self, _key: &K) {}
}
