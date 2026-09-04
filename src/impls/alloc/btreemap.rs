// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use alloc_::collections::BTreeMap;
use alloc_::collections::btree_map::{
    Entry as BTreeMapEntry, OccupiedEntry as BTreeMapOccupiedEntry,
    VacantEntry as BTreeMapVacantEntry,
};

use crate::abc::{Container, Keyed};
use crate::entry::{CombinedEntry, Entry, OccupiedEntry, VacantEntry};
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Insert, Len, Modify, Remove, Set};

impl<K, V> Container for BTreeMap<K, V> {
    type Value = V;
}

impl<K, V> Keyed for BTreeMap<K, V> {
    type Key = K;
}

impl<K: Ord, Q: Ord + ?Sized, V> Get<Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        BTreeMap::get(self, key)
    }
}

impl<K: Ord, V> Set<K> for BTreeMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        BTreeMap::insert(self, key, value)
    }
}

impl<K: Ord, Q: Ord + ?Sized, V> Modify<Q> for BTreeMap<K, V>
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

impl<K: Ord, V> Insert<K> for BTreeMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        BTreeMap::insert(self, key, value)
    }
}

impl<K: Ord, Q: Ord + ?Sized, V> Remove<Q> for BTreeMap<K, V>
where
    K: Borrow<Q>,
{
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        BTreeMap::remove(self, key)
    }
}

impl<K: Ord, V> Clear for BTreeMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        BTreeMap::clear(self);
    }
}

impl<K, V> Len for BTreeMap<K, V> {
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeMap::len(self)
    }
}

impl<K: Ord, V> Entry<K> for BTreeMap<K, V> {
    type Entry<'a>
        = BTreeMapEntry<'a, K, V>
    where
        Self: 'a,
        K: 'a;

    #[inline(always)]
    fn entry(&mut self, key: K) -> Self::Entry<'_> {
        BTreeMap::entry(self, key)
    }
}

impl<'a, K: Ord, V> CombinedEntry<'a, K, V> for BTreeMapEntry<'a, K, V> {
    type OccupiedEntry = BTreeMapOccupiedEntry<'a, K, V>;

    #[inline(always)]
    fn key(&self) -> &K {
        BTreeMapEntry::key(self)
    }

    #[inline(always)]
    fn or_insert(self, default: V) -> &'a mut V {
        BTreeMapEntry::or_insert(self, default)
    }

    #[inline(always)]
    fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        BTreeMapEntry::or_insert_with(self, default)
    }

    #[inline(always)]
    fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V,
    {
        BTreeMapEntry::or_insert_with_key(self, default)
    }

    #[inline(always)]
    fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        BTreeMapEntry::and_modify(self, f)
    }

    #[inline(always)]
    fn insert_entry(self, value: V) -> Self::OccupiedEntry {
        BTreeMapEntry::insert_entry(self, value)
    }

    #[inline(always)]
    fn or_default(self) -> &'a mut V
    where
        V: Default,
    {
        BTreeMapEntry::or_default(self)
    }
}

impl<'a, K: Ord, V> OccupiedEntry<'a, K, V> for BTreeMapOccupiedEntry<'a, K, V> {
    #[inline(always)]
    fn key(&self) -> &K {
        BTreeMapOccupiedEntry::key(self)
    }

    #[inline(always)]
    fn get(&self) -> &V {
        BTreeMapOccupiedEntry::get(self)
    }

    #[inline(always)]
    fn get_mut(&mut self) -> &mut V {
        BTreeMapOccupiedEntry::get_mut(self)
    }

    #[inline(always)]
    fn into_mut(self) -> &'a mut V {
        BTreeMapOccupiedEntry::into_mut(self)
    }

    #[inline(always)]
    fn insert(&mut self, value: V) -> V {
        BTreeMapOccupiedEntry::insert(self, value)
    }

    #[inline(always)]
    fn remove(self) -> V {
        BTreeMapOccupiedEntry::remove(self)
    }

    #[inline(always)]
    fn remove_entry(self) -> (K, V) {
        BTreeMapOccupiedEntry::remove_entry(self)
    }
}

impl<'a, K: Ord, V> VacantEntry<'a, K, V> for BTreeMapVacantEntry<'a, K, V> {
    type OccupiedEntry = BTreeMapOccupiedEntry<'a, K, V>;

    #[inline(always)]
    fn key(&self) -> &K {
        BTreeMapVacantEntry::key(self)
    }

    #[inline(always)]
    fn into_key(self) -> K {
        BTreeMapVacantEntry::into_key(self)
    }

    #[inline(always)]
    fn insert(self, value: V) -> &'a mut V {
        BTreeMapVacantEntry::insert(self, value)
    }

    #[inline(always)]
    fn insert_entry(self, value: V) -> Self::OccupiedEntry {
        BTreeMapVacantEntry::insert_entry(self, value)
    }
}

impl<'a, K: 'a, V: 'a> Values<'a> for BTreeMap<K, V> {
    type Values = alloc_::collections::btree_map::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        BTreeMap::values(self)
    }
}

impl<K, V> IntoValues for BTreeMap<K, V> {
    type IntoValues = alloc_::collections::btree_map::IntoValues<K, V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        BTreeMap::into_values(self)
    }
}

impl<'a, K: 'a, V: 'a> Iter<'a, &'a K> for BTreeMap<K, V> {
    type Iter = alloc_::collections::btree_map::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        BTreeMap::iter(self)
    }
}

impl<K, V> IntoIter<K> for BTreeMap<K, V> {
    type IntoIter = alloc_::collections::btree_map::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> alloc_::collections::btree_map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
