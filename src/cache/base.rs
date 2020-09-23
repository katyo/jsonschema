use crate::Args;
use crypto_hashes::{digest::Digest, sha3::Sha3_256};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "sled")]
use super::sled::Db;

#[cfg(not(feature = "sled"))]
use super::file::Db;

/// Cache store
pub struct Cache {
    db: Option<Db>,
}

impl Cache {
    pub fn open(args: &Args, group: impl AsRef<str>) -> Self {
        let group = group.as_ref();
        let db = if args.no_cache {
            None
        } else {
            let path = args.cache_dir.as_ref().unwrap().join(group);
            Self::_open(&path)
        };
        Self { db }
    }

    /// Get data from cache
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: Serialize,
        V: DeserializeOwned,
    {
        let key = _serialize_key(key)?;
        let db = self.db.as_ref()?;
        let val = Self::_get(db, &key)?;
        json::from_slice(val.as_ref())
            .map_err(|error| {
                log::error!("Unable to parse cached value due to: {}", error);
            })
            .ok()
    }

    fn put_<K, V>(&self, key: &K, val: &V) -> Option<()>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = _serialize_key(key)?;
        let val = _serialize_val(val)?;
        let db = self.db.as_ref()?;
        Self::_put(db, &key, &val)
    }

    /// Store data into cache
    pub fn put<K, V>(&self, key: &K, val: &V)
    where
        K: Serialize,
        V: Serialize,
    {
        let _ = self.put_(key, val);
    }

    fn del_<K>(&self, key: &K) -> Option<()>
    where
        K: Serialize,
    {
        let key = _serialize_key(key)?;
        let db = self.db.as_ref()?;
        Self::_del(db, &key)
    }

    /// Remove data from cache
    pub fn del<K>(&self, key: &K)
    where
        K: Serialize,
    {
        let _ = self.del_(key);
    }
}

fn _serialize_key<K>(key: &K) -> Option<Vec<u8>>
where
    K: Serialize,
{
    json::to_vec(key)
        .map_err(|error| log::error!("Unable to serialize cache key due to: {}", error))
        .ok()
        .map(|key| {
            let mut hasher = Sha3_256::default();
            hasher.update(&key);
            let hash = hasher.finalize();
            Vec::from(&hash as &[u8])
        })
}

fn _serialize_val<V>(val: &V) -> Option<Vec<u8>>
where
    V: Serialize,
{
    json::to_vec(val)
        .map_err(|error| log::error!("Unable to serialize cache value due to: {}", error))
        .ok()
}
