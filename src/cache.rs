/*!

Cache management

*/

use crate::Args;

#[cfg(feature = "cache")]
mod base;

#[cfg(not(feature = "cache"))]
mod stub;

#[cfg(all(feature = "cache", feature = "sled"))]
mod sled;

#[cfg(all(feature = "cache", not(feature = "sled")))]
mod file;

#[cfg(feature = "cache")]
pub use base::Cache;

#[cfg(not(feature = "cache"))]
pub use stub::Cache;

impl Args {
    #[cfg(feature = "cache")]
    pub fn fix_cache(&mut self) {
        if self.no_cache {
            self.cache_dir = None;
        } else if self.cache_dir.is_none() {
            self.cache_dir = dirs::cache_dir().map(|path| path.join(env!("CARGO_PKG_NAME")));
            if self.cache_dir.is_none() {
                self.no_cache = true;
                log::warn!("Unable to guess cache directory");
            }
        }
    }

    #[cfg(not(feature = "cache"))]
    pub fn fix_cache(&self) {}
}
