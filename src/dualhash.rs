use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
pub struct DualHashMap<K1, K2, V>
where
    K1: std::hash::Hash + Eq + Send + Clone,
    K2: std::hash::Hash + Eq + Send + Clone,
{
    data_map: HashMap<K1, V>,
    sec_key_map: HashMap<K2, HashSet<K1>>,
}

impl<K1, K2, V> DualHashMap<K1, K2, V>
where
    K1: std::hash::Hash + Eq + Send + Clone,
    K2: std::hash::Hash + Eq + Send + Clone,
{
    pub fn new() -> Self {
        DualHashMap {
            data_map: HashMap::new(),
            sec_key_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k1: K1, k2: K2, value: V) {
        for set in self.sec_key_map.values_mut() {
            set.remove(&k1);
        }
        self.sec_key_map.retain(|_, set| !set.is_empty());

        self.data_map.insert(k1.clone(), value);
        self.sec_key_map.entry(k2).or_default().insert(k1);
    }

    pub fn get_by_k1(&self, k1: &K1) -> Option<&V> {
        self.data_map.get(k1)
    }

    pub fn get_by_k2(&self, k2: &K2) -> impl Iterator<Item = &V> {
        self.sec_key_map
            .get(k2)
            .into_iter()
            .flat_map(|k1s| k1s.iter())
            .filter_map(|k1| self.data_map.get(k1))
    }

    pub fn get_mut_by_k1(&mut self, k1: &K1) -> Option<&mut V> {
        self.data_map.get_mut(k1)
    }

    pub fn for_each_mut_by_k2<F>(&mut self, k2: &K2, mut f: F)
    where
        F: FnMut(&mut V),
    {
        if let Some(k1s) = self.sec_key_map.get(k2) {
            for k1 in k1s.iter() {
                if let Some(v) = self.data_map.get_mut(k1) {
                    f(v);
                }
            }
        }
    }

    pub fn get_values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.data_map.values_mut()
    }
    pub fn get_values(&self) -> impl Iterator<Item = &V> {
        self.data_map.values()
    }
    pub fn get_keys1(&self) -> impl Iterator<Item = &K1> {
        self.data_map.keys()
    }
    pub fn get_keys2(&self) -> impl Iterator<Item = &K2> {
        self.sec_key_map.keys()
    }

    pub fn remove_by_k1(&mut self, k1: &K1) {
        if self.data_map.remove(k1).is_some() {
            for (_, k1_set) in self.sec_key_map.iter_mut() {
                k1_set.remove(k1);
            }
            self.sec_key_map.retain(|_, set| !set.is_empty());
        }
    }

    pub fn remove_by_k2(&mut self, k2: &K2) {
        if let Some(k1s) = self.sec_key_map.remove(k2) {
            for k1 in k1s {
                self.data_map.remove(&k1);
            }
        }
    }

    pub fn contains_k1(&self, k1: &K1) -> bool {
        self.data_map.contains_key(k1)
    }

    pub fn contains_k2(&self, k2: &K2) -> bool {
        self.sec_key_map.contains_key(k2)
    }

    pub fn get_k1s_by_k2(&self, k2: &K2) -> Option<&HashSet<K1>> {
        self.sec_key_map.get(k2)
    }

    pub fn len(&self) -> usize {
        self.data_map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data_map.is_empty()
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
