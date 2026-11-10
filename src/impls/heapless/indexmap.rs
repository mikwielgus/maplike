// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

use heapless::IndexMap;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{CheckedInsert, Clear, ContainsKey, Get, Len, Modify, Remove, Set};

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> Container for IndexMap<K, V, S, N> {
    type Value = V;
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> Keyed for IndexMap<K, V, S, N> {
    type Key = K;
}

impl<K: Eq + Hash, Q, V, S: BuildHasher, const N: usize> ContainsKey<Q> for IndexMap<K, V, S, N>
where
    K: Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        IndexMap::contains_key(self, key)
    }
}

impl<K: Eq + Hash, Q, V, S: BuildHasher, const N: usize> Get<Q> for IndexMap<K, V, S, N>
where
    K: Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        IndexMap::get(self, key)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> Set<K> for IndexMap<K, V, S, N> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        let slot = IndexMap::get_mut(self, &key).expect("no value under key");

        Some(core::mem::replace(slot, value))
    }
}

impl<K: Eq + Hash, Q, V, S: BuildHasher, const N: usize> Modify<Q> for IndexMap<K, V, S, N>
where
    K: Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    #[inline(always)]
    fn modify<F>(&mut self, key: &Q, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(IndexMap::get_mut(self, key).expect("no value under key"));
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> CheckedInsert<K> for IndexMap<K, V, S, N> {
    type Output = Option<V>;

    #[inline(always)]
    fn checked_insert(&mut self, key: K, value: V) -> Option<Self::Output> {
        IndexMap::insert(self, key, value).ok()
    }
}

impl<K: Eq + Hash, Q, V, S: BuildHasher, const N: usize> Remove<Q> for IndexMap<K, V, S, N>
where
    K: Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        IndexMap::remove(self, key)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> Clear for IndexMap<K, V, S, N> {
    #[inline(always)]
    fn clear(&mut self) {
        IndexMap::clear(self);
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> Len for IndexMap<K, V, S, N> {
    #[inline(always)]
    fn len(&self) -> usize {
        IndexMap::len(self)
    }
}

impl<'a, K: Eq + Hash + 'a, V: 'a, S: BuildHasher, const N: usize> Values<'a>
    for IndexMap<K, V, S, N>
where
    S: 'a,
{
    type Values = heapless::index_map::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        IndexMap::values(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> IntoValues for IndexMap<K, V, S, N> {
    type IntoValues = ValuesFromKeyValuePairs<heapless::index_map::IntoIter<K, V, N>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, K: Eq + Hash + 'a, V: 'a, S: BuildHasher, const N: usize> Iter<'a, &'a K>
    for IndexMap<K, V, S, N>
where
    S: 'a,
{
    type Iter = heapless::index_map::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        IndexMap::iter(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> IntoIter<K> for IndexMap<K, V, S, N> {
    type IntoIter = heapless::index_map::IntoIter<K, V, N>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}
