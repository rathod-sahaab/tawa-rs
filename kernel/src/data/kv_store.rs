extern crate alloc;
use alloc::string::String;

/// Key-value store abstraction with string keys of the form "prefix_id".
pub trait KvStore<V> {
    /// Insert a value, returning the generated key as a String
    fn insert(&mut self, value: V) -> String;
    /// Retrieve a value by id string of the form "prefix_id"
    fn get(&self, id: &str) -> Option<&V>;
    /// Remove a value by id string of the form "prefix_id"
    fn remove(&mut self, id: &str) -> bool;
}
