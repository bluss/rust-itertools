#![cfg(feature = "use_std")]

use fnv::FnvHashMap;
use std::hash::Hash;
use std::iter::Iterator;

/// Return a `HashMap` of keys mapped to a list of their corresponding values.
///
/// See [`.into_group_map()`](../trait.Itertools.html#method.into_group_map)
/// for more information.
pub fn into_group_map<I, K, V>(iter: I) -> FnvHashMap<K, Vec<V>>
where
    I: Iterator<Item = (K, V)>,
    K: Hash + Eq,
{
    let mut lookup = FnvHashMap::default();

    for (key, val) in iter {
        lookup.entry(key).or_insert(Vec::new()).push(val);
    }

    lookup
}
