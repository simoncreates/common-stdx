use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct DualHashMap<K1, K2, V>
where
    K1: std::hash::Hash + Eq + Send + Clone,
    K2: std::hash::Hash + Eq + Send + Clone,
{
    data_map: HashMap<K1, V>,
    sec_key_map: HashMap<K2, K1>,
}

impl<K1, K2, V> DualHashMap<K1, K2, V>
where
    K1: std::hash::Hash + Eq + Send + Clone,
    K2: std::hash::Hash + Eq + Send + Clone,
{
    fn new() -> Self {
        DualHashMap {
            data_map: HashMap::new(),
            sec_key_map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, k1: K1, k2: K2, value: V) {
        self.data_map.insert(k1.clone(), value);
        self.sec_key_map.insert(k2, k1);
    }

    pub fn get_by_k1(&self, k1: &K1) -> Option<&V> {
        self.data_map.get(k1)
    }

    pub fn get_by_k2(&self, k2: &K2) -> Option<&V> {
        self.sec_key_map.get(k2).and_then(|k1| self.get_by_k1(k1))
    }

    fn remove_by_k1(&mut self, k1: &K1) {
        if self.data_map.remove(k1).is_some() {
            self.sec_key_map.retain(|_, v| v != k1);
        }
    }

    fn remove_by_k2(&mut self, k2: &K2) {
        if let Some(k1) = self.sec_key_map.remove(k2) {
            self.data_map.remove(&k1);
        }
    }
}

impl<K1, K2, V> Default for DualHashMap<K1, K2, V>
where
    K1: std::hash::Hash + Eq + Send + Clone,
    K2: std::hash::Hash + Eq + Send + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
