use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct MetaTable<O> {
    prototype: Option<Arc<Mutex<Self>>>,
    args: Vec<O>,
    kws: BTreeMap<String, O>,
}

impl<O> MetaTable<O> {
    pub fn new(capacity: usize) -> Self {
        Self { prototype: None, args: Vec::with_capacity(capacity), kws: Default::default() }
    }
    pub fn insert_item<T>(&mut self, value: T)
    where
        T: Into<O>,
    {
        self.args.push(value.into());
    }
    pub fn insert_pair<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<O>,
    {
        self.kws.insert(key.into(), value.into());
    }
    pub fn extend(&mut self, other: Self) {
        self.args.extend(other.args);
        self.kws.extend(other.kws);
    }
    pub fn get_item(&self, index: usize) -> Option<&O> {
        self.args.get(index)
    }
    pub fn get_key(&self, key: &str) -> Option<&O> {
        self.kws.get(key)
    }
}

impl<V, O> FromIterator<V> for MetaTable<O>
where
    V: Into<O>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self { prototype: None, args: iter.into_iter().map(|v| v.into()).collect(), kws: Default::default() }
    }
}
