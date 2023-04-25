use crate::VirtualObject;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct VirtualTable {
    args: Vec<VirtualObject>,
    kws: BTreeMap<String, VirtualObject>,
}

impl VirtualTable {
    pub fn new(capacity: usize) -> Self {
        Self { args: Vec::with_capacity(capacity), kws: Default::default() }
    }
    pub fn insert_item<T>(&mut self, value: T)
    where
        T: Into<VirtualObject>,
    {
        self.args.push(value.into());
    }
    pub fn insert_pair<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<VirtualObject>,
    {
        self.kws.insert(key.into(), value.into());
    }
    pub fn extend<T>(&mut self, other: Self)
    where
        T: Into<VirtualTable>,
    {
        self.args.extend(other.args);
        self.kws.extend(other.kws);
    }
    pub fn get_item(&self, index: usize) -> Option<&VirtualObject> {
        self.args.get(index)
    }
    pub fn get_key(&self, key: &str) -> Option<&VirtualObject> {
        self.kws.get(key)
    }
}

impl<V> FromIterator<V> for VirtualTable
where
    V: Into<VirtualObject>,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self { args: iter.into_iter().map(|v| v.into()).collect(), kws: Default::default() }
    }
}

impl<K, V> FromIterator<(K, V)> for VirtualTable
where
    K: Into<String>,
    V: Into<VirtualObject>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self { args: vec![], kws: iter.into_iter().map(|(k, v)| (k.into(), v.into())).collect() }
    }
}
