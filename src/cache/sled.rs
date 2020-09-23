use super::Cache;
use std::path::Path;

pub type Db = sled::Db;
pub type Raw = sled::IVec;

impl Cache {
    pub(super) fn _open(path: &Path) -> Option<Db> {
        sled::open(path)
            .map_err(|error| {
                log::error!(
                    "Unablte to open cache database '{}' due to: {}",
                    path.display(),
                    error
                )
            })
            .ok()
    }

    pub(super) fn _get(db: &Db, key: &[u8]) -> Option<Raw> {
        db.get(&key)
            .map_err(|error| {
                log::error!("Unable to get from cache due to: {}", error);
            })
            .ok()?
    }

    pub(super) fn _put(db: &Db, key: &[u8], val: &[u8]) -> Option<()> {
        db.insert(key, val)
            .map_err(|error| log::error!("Unable to insert into cache due to: {}", error))
            .ok()
            .map(|_| {})
    }

    pub(super) fn _del(db: &Db, key: &[u8]) -> Option<()> {
        db.remove(&key)
            .map_err(|error| log::error!("Unable to remove from cache due to: {}", error))
            .ok()
            .map(|_| {})
    }
}
