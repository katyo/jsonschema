/*!

File-based cacher backend implementation

*/

use super::Cache;
use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub type Db = PathBuf;
pub type Raw = Vec<u8>;

impl Cache {
    pub(super) fn _open(path: &Path) -> Option<Db> {
        if !path.is_dir() {
            std::fs::create_dir_all(path)
                .map_err(|error| {
                    log::error!(
                        "Unable to create directory '{}' due to: {}",
                        path.display(),
                        error
                    )
                })
                .ok()?;
        }
        Some(path.into())
    }

    pub(super) fn _get(db: &Db, key: &[u8]) -> Option<Raw> {
        let path = _key_path(db, key);
        if path.is_file() {
            let mut file = File::open(&path)
                .map_err(|error| {
                    log::error!(
                        "Unable to open cache data file '{}' due to: {}",
                        path.display(),
                        error
                    );
                })
                .ok()?;

            let mut data = Vec::new();
            file.read_to_end(&mut data)
                .map_err(|error| {
                    log::error!(
                        "Unable to read cache data file '{}' due to: {}",
                        path.display(),
                        error
                    );
                })
                .ok()?;
            Some(data)
        } else {
            None
        }
    }

    pub(super) fn _put(db: &Db, key: &[u8], val: &[u8]) -> Option<()> {
        let path = _key_path(db, key);

        let mut file = File::create(&path)
            .map_err(|error| {
                log::error!(
                    "Unable to create cache data file '{}' due to: {}",
                    path.display(),
                    error
                )
            })
            .ok()?;

        file.write_all(val)
            .map_err(|error| {
                log::error!(
                    "Unable to write cache data file '{}' due to: {}",
                    path.display(),
                    error
                )
            })
            .ok()
    }

    pub(super) fn _del(db: &Db, key: &[u8]) -> Option<()> {
        let path = _key_path(db, key);
        if path.is_file() {
            std::fs::remove_file(&path)
                .map_err(|error| {
                    log::error!(
                        "Unable to remove cache data file '{}' due to: {}",
                        path.display(),
                        error
                    );
                })
                .ok()
        } else {
            None
        }
    }
}

fn _key_path(db: &Db, key: &[u8]) -> PathBuf {
    use base64::Engine;

    let key = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(key);
    db.join(key)
}
