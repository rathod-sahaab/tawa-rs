extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use super::kv_store::KvStore;

/// A simple, no_std+alloc key-value store using BTreeMap and static prefix-checked keys.
pub struct KvStoreBTreeImpl<V> {
    pub map: BTreeMap<u64, V>,
    next_id: u64,
    prefix: &'static str,
}

impl<V> KvStoreBTreeImpl<V> {
    pub fn new(prefix: &'static str) -> Self {
        Self { map: BTreeMap::new(), next_id: 0, prefix }
    }
    fn parse_prefix_id(&self, id: &str) -> Option<(String, u64)> {
        let mut parts = id.rsplitn(2, '_');
        let id_part = parts.next()?;
        let prefix = parts.next()?.to_string();
        let id_num = id_part.parse().ok()?;
        Some((prefix, id_num))
    }
}

impl<V> KvStore<V> for KvStoreBTreeImpl<V> {
    fn insert(&mut self, value: V) -> String {
        let id = self.next_id;
        self.next_id += 1;
        let key = alloc::format!("{}_{}", self.prefix, id);
        self.map.insert(id, value);
        key
    }
    fn get(&self, id: &str) -> Option<&V> {
        let (prefix, id_num) = self.parse_prefix_id(id)?;
        if prefix != self.prefix {
            return None;
        }
        self.map.get(&id_num)
    }
    fn remove(&mut self, id: &str) -> bool {
        let (prefix, id_num) = match self.parse_prefix_id(id) {
            Some(x) => x,
            None => return false,
        };
        if prefix != self.prefix {
            return false;
        }
        self.map.remove(&id_num).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::KvStore;
    #[test]
    fn test_insert_and_get() {
        let mut store = KvStoreBTreeImpl::new("curve");
        let id = store.insert(42);
        assert_eq!(store.get("curve_0"), Some(&42));
        assert_eq!(store.get("other_0"), None); // wrong prefix
    }

    #[test]
    fn test_remove() {
        let mut store = KvStoreBTreeImpl::new("foo");
        let id = store.insert(123);
        assert_eq!(store.get("foo_0"), Some(&123));
        assert!(store.remove("foo_0"));
        assert_eq!(store.get("foo_0"), None);
        assert!(!store.remove("foo_0")); // already removed
    }

    #[test]
    fn test_multiple_inserts() {
        let mut store = KvStoreBTreeImpl::new("bar");
        let id1 = store.insert(1);
        let id2 = store.insert(2);
        assert_ne!(id1, id2);
        assert_eq!(store.get("bar_0"), Some(&1));
        assert_eq!(store.get("bar_1"), Some(&2));
    }
}
