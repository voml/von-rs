use crate::VirtualObject;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtualKey {
    wrapped: String,
}

pub struct VirtualTable {
    args: Vec<VirtualObject>,
    kws: BTreeMap<VirtualKey, VirtualObject>,
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
    K: Into<VirtualKey>,
    V: Into<VirtualObject>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self { args: vec![], kws: iter.into_iter().map(|(k, v)| (k.into(), v.into())).collect() }
    }
}
