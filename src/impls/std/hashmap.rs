// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

use std_::collections::HashMap;
use std_::collections::hash_map::{
    Entry as HashMapEntry, OccupiedEntry as HashMapOccupiedEntry, VacantEntry as HashMapVacantEntry,
};

use crate::abc::{Container, Keyed};
use crate::ctors::{WithCapacity, WithCapacityAndHasher, WithHasher};
use crate::entry::{CombinedEntry, Entry, OccupiedEntry, VacantEntry};
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, ContainsKey, Get, Insert, Len, Modify, Remove, Set};

impl<K, V, S> Container for HashMap<K, V, S> {
    type Value = V;
}

impl<K, V, S> Keyed for HashMap<K, V, S> {
    type Key = K;
}

impl<K, V, S: BuildHasher + Default> WithCapacity for HashMap<K, V, S> {
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K, V, S> WithHasher<S> for HashMap<K, V, S> {
    #[inline(always)]
    fn with_hasher(hasher: S) -> Self {
        HashMap::with_hasher(hasher)
    }
}

impl<K, V, S> WithCapacityAndHasher<S> for HashMap<K, V, S> {
    #[inline(always)]
    fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        HashMap::with_capacity_and_hasher(capacity, hasher)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> ContainsKey<Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        HashMap::contains_key(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> Get<Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        HashMap::get(self, key)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Set<K> for HashMap<K, V, S> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> Modify<Q> for HashMap<K, V, S>
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

impl<K: Eq + Hash, V, S: BuildHasher> Insert<K> for HashMap<K, V, S> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        HashMap::insert(self, key, value)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, V, S: BuildHasher> Remove<Q> for HashMap<K, V, S>
where
    K: Borrow<Q>,
{
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        HashMap::remove(self, key)
    }
}

impl<K, V, S: BuildHasher> Clear for HashMap<K, V, S> {
    #[inline(always)]
    fn clear(&mut self) {
        HashMap::clear(self);
    }
}

impl<K, V, S> Len for HashMap<K, V, S> {
    #[inline(always)]
    fn len(&self) -> usize {
        HashMap::len(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> Entry<K> for HashMap<K, V, S> {
    type Entry<'a>
        = HashMapEntry<'a, K, V>
    where
        Self: 'a,
        K: 'a;

    #[inline(always)]
    fn entry(&mut self, key: K) -> Self::Entry<'_> {
        HashMap::entry(self, key)
    }
}

impl<'a, K: Eq + Hash, V> CombinedEntry<'a, K, V> for HashMapEntry<'a, K, V> {
    type OccupiedEntry = HashMapOccupiedEntry<'a, K, V>;

    #[inline(always)]
    fn key(&self) -> &K {
        HashMapEntry::key(self)
    }

    #[inline(always)]
    fn or_insert(self, default: V) -> &'a mut V {
        HashMapEntry::or_insert(self, default)
    }

    #[inline(always)]
    fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        HashMapEntry::or_insert_with(self, default)
    }

    #[inline(always)]
    fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        HashMapEntry::or_insert_with_key(self, default)
    }

    #[inline(always)]
    fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        HashMapEntry::and_modify(self, f)
    }

    #[inline(always)]
    fn insert_entry(self, value: V) -> Self::OccupiedEntry {
        HashMapEntry::insert_entry(self, value)
    }

    #[inline(always)]
    fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        HashMapEntry::or_default(self)
    }
}

impl<'a, K: Eq + Hash, V> OccupiedEntry<'a, K, V> for HashMapOccupiedEntry<'a, K, V> {
    #[inline(always)]
    fn key(&self) -> &K {
        HashMapOccupiedEntry::key(self)
    }

    #[inline(always)]
    fn get(&self) -> &V {
        HashMapOccupiedEntry::get(self)
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut V {
        HashMapOccupiedEntry::get_mut(self)
    }

    #[inline(always)]
    fn into_mut(self) -> &'a mut V {
        HashMapOccupiedEntry::into_mut(self)
    }

    #[inline(always)]
    fn insert(&mut self, value: V) -> V {
        HashMapOccupiedEntry::insert(self, value)
    }

    #[inline(always)]
    fn remove(self) -> V {
        HashMapOccupiedEntry::remove(self)
    }

    #[inline(always)]
    fn remove_entry(self) -> (K, V) {
        HashMapOccupiedEntry::remove_entry(self)
    }
}

impl<'a, K: Eq + Hash, V> VacantEntry<'a, K, V> for HashMapVacantEntry<'a, K, V> {
    type OccupiedEntry = HashMapOccupiedEntry<'a, K, V>;

    #[inline(always)]
    fn key(&self) -> &K {
        HashMapVacantEntry::key(self)
    }

    #[inline(always)]
    fn into_key(self) -> K {
        HashMapVacantEntry::into_key(self)
    }

    #[inline(always)]
    fn insert(self, value: V) -> &'a mut V {
        HashMapVacantEntry::insert(self, value)
    }

    #[inline(always)]
    fn insert_entry(self, value: V) -> Self::OccupiedEntry {
        HashMapVacantEntry::insert_entry(self, value)
    }
}

impl<'a, K: 'a, V: 'a, S: 'a> Values<'a> for HashMap<K, V, S> {
    type Values = std_::collections::hash_map::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        HashMap::values(self)
    }
}

impl<K, V, S> IntoValues for HashMap<K, V, S> {
    type IntoValues = std_::collections::hash_map::IntoValues<K, V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        HashMap::into_values(self)
    }
}

impl<'a, K: 'a, V: 'a, S: 'a> Iter<'a, &'a K> for HashMap<K, V, S> {
    type Iter = std_::collections::hash_map::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        HashMap::iter(self)
    }
}

impl<K, V, S> IntoIter<K> for HashMap<K, V, S> {
    type IntoIter = std_::collections::hash_map::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> std_::collections::hash_map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
