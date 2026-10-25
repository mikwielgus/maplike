// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

use indexmap::IndexMap;
use indexmap::map::{
    Entry as IndexMapEntry, OccupiedEntry as IndexMapOccupiedEntry,
    VacantEntry as IndexMapVacantEntry,
};

use crate::abc::{Container, Keyed};
use crate::entry::{CombinedEntry, Entry, OccupiedEntry, VacantEntry};
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{
    Clear, ContainsKey, Get, Insert, Len, Modify, Remove, Set, WithCapacity, WithCapacityAndHasher,
    WithHasher,
};

impl<K, V, S> Container for IndexMap<K, V, S> {
    type Value = V;
}

impl<K, V, S> Keyed for IndexMap<K, V, S> {
    type Key = K;
}

impl<K, V, S: BuildHasher + Default> WithCapacity for IndexMap<K, V, S> {
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self {
        IndexMap::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K, V, S> WithHasher<S> for IndexMap<K, V, S> {
    #[inline(always)]
    fn with_hasher(hasher: S) -> Self {
        IndexMap::with_hasher(hasher)
    }
}

impl<K, V, S> WithCapacityAndHasher<S> for IndexMap<K, V, S> {
    #[inline(always)]
    fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        IndexMap::with_capacity_and_hasher(capacity, hasher)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> ContainsKey<Q> for IndexMap<K, V, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        IndexMap::contains_key(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> Get<Q> for IndexMap<K, V, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        IndexMap::get(self, key)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Set<K> for IndexMap<K, V, S> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        IndexMap::insert(self, key, value)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> Modify<Q> for IndexMap<K, V, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn modify<F>(&mut self, key: &Q, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(key).expect("no value under key"));
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Insert<K> for IndexMap<K, V, S> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        IndexMap::insert(self, key, value)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> Remove<Q> for IndexMap<K, V, S>
where
    K: Borrow<Q>,
{
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        IndexMap::shift_remove(self, key)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Clear for IndexMap<K, V, S> {
    #[inline(always)]
    fn clear(&mut self) {
        IndexMap::clear(self);
    }
}

impl<K, V, S> Len for IndexMap<K, V, S> {
    #[inline(always)]
    fn len(&self) -> usize {
        IndexMap::len(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Entry<K> for IndexMap<K, V, S> {
    type Entry<'a>
        = IndexMapEntry<'a, K, V>
    where
        Self: 'a,
        K: 'a;

    #[inline(always)]
    fn entry(&mut self, key: K) -> Self::Entry<'_> {
        IndexMap::entry(self, key)
    }
}

impl<'a, K: Eq + Hash, V> CombinedEntry<'a, K, V> for IndexMapEntry<'a, K, V> {
    type OccupiedEntry = IndexMapOccupiedEntry<'a, K, V>;

    #[inline(always)]
    fn key(&self) -> &K {
        IndexMapEntry::key(self)
    }

    #[inline(always)]
    fn or_insert(self, default: V) -> &'a mut V {
        IndexMapEntry::or_insert(self, default)
    }

    #[inline(always)]
    fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        IndexMapEntry::or_insert_with(self, default)
    }

    #[inline(always)]
    fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        IndexMapEntry::or_insert_with_key(self, default)
    }

    #[inline(always)]
    fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        IndexMapEntry::and_modify(self, f)
    }

    #[inline(always)]
    fn insert_entry(self, value: V) -> Self::OccupiedEntry {
        IndexMapEntry::insert_entry(self, value)
    }

    #[inline(always)]
    fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        IndexMapEntry::or_default(self)
    }
}

impl<'a, K: Eq + Hash, V> OccupiedEntry<'a, K, V> for IndexMapOccupiedEntry<'a, K, V> {
    #[inline(always)]
    fn key(&self) -> &K {
        IndexMapOccupiedEntry::key(self)
    }

    #[inline(always)]
    fn get(&self) -> &V {
        IndexMapOccupiedEntry::get(self)
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut V {
        IndexMapOccupiedEntry::get_mut(self)
    }

    #[inline(always)]
    fn into_mut(self) -> &'a mut V {
        IndexMapOccupiedEntry::into_mut(self)
    }

    #[inline(always)]
    fn insert(&mut self, value: V) -> V {
        IndexMapOccupiedEntry::insert(self, value)
    }

    #[inline(always)]
    fn remove(self) -> V {
        IndexMapOccupiedEntry::shift_remove(self)
    }

    #[inline(always)]
    fn remove_entry(self) -> (K, V) {
        IndexMapOccupiedEntry::shift_remove_entry(self)
    }
}

impl<'a, K: Eq + Hash, V> VacantEntry<'a, K, V> for IndexMapVacantEntry<'a, K, V> {
    type OccupiedEntry = IndexMapOccupiedEntry<'a, K, V>;

    #[inline(always)]
    fn key(&self) -> &K {
        IndexMapVacantEntry::key(self)
    }

    #[inline(always)]
    fn into_key(self) -> K {
        IndexMapVacantEntry::into_key(self)
    }

    #[inline(always)]
    fn insert(self, value: V) -> &'a mut V {
        IndexMapVacantEntry::insert(self, value)
    }

    #[inline(always)]
    fn insert_entry(self, value: V) -> Self::OccupiedEntry {
        IndexMapVacantEntry::insert_entry(self, value)
    }
}

impl<'a, K: 'a, V: 'a, S: 'a> Values<'a> for IndexMap<K, V, S> {
    type Values = indexmap::map::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        IndexMap::values(self)
    }
}

impl<K, V, S> IntoValues for IndexMap<K, V, S> {
    type IntoValues = indexmap::map::IntoValues<K, V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IndexMap::into_values(self)
    }
}

impl<'a, K: 'a, V: 'a, S: 'a> Iter<'a, &'a K> for IndexMap<K, V, S> {
    type Iter = indexmap::map::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        IndexMap::iter(self)
    }
}

impl<K, V, S> IntoIter<K> for IndexMap<K, V, S> {
    type IntoIter = indexmap::map::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> indexmap::map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
