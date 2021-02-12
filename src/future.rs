//! Provides thread-safe, asynchronous (futures aware) cache implementations.
//!
//! To use this module, enable a crate feature called "future".

mod builder;
pub(crate) mod cache;

pub use builder::CacheBuilder;
pub use cache::Cache;

/// Provides extra methods that will be useful for testing.
pub trait ConcurrentCacheExt<K, V> {
    /// Performs any pending maintenance operations needed by the cache.
    fn sync(&self);
}
