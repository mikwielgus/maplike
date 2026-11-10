// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

use heapless::IndexSet;

use crate::abc::{Container, Keyed};
use crate::iter::{Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{CheckedInsert, Clear, ContainsKey, Get, Len, Remove};

impl<K: Eq + Hash, S: BuildHasher, const N: usize> Container for IndexSet<K, S, N> {
    type Value = ();
}

impl<K: Eq + Hash, S: BuildHasher, const N: usize> Keyed for IndexSet<K, S, N> {
    type Key = K;
}

impl<K: Eq + Hash, Q, S: BuildHasher, const N: usize> ContainsKey<Q> for IndexSet<K, S, N>
where
    K: Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        IndexSet::contains(self, key)
    }
}

impl<K: Eq + Hash, Q, S: BuildHasher, const N: usize> Get<Q> for IndexSet<K, S, N>
where
    K: Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        IndexSet::contains(self, key).then_some(&())
    }
}

impl<K: Eq + Hash, S: BuildHasher, const N: usize> CheckedInsert<K> for IndexSet<K, S, N> {
    type Output = bool;

    #[inline(always)]
    fn checked_insert(&mut self, key: K, _value: ()) -> Option<Self::Output> {
        IndexSet::insert(self, key).ok()
    }
}

impl<K: Eq + Hash, Q, S: BuildHasher, const N: usize> Remove<Q> for IndexSet<K, S, N>
where
    K: Borrow<Q>,
    Q: Eq + Hash + ?Sized,
{
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        IndexSet::remove(self, key).then_some(())
    }
}

impl<K: Eq + Hash, S: BuildHasher, const N: usize> Clear for IndexSet<K, S, N> {
    #[inline(always)]
    fn clear(&mut self) {
        IndexSet::clear(self);
    }
}

impl<K: Eq + Hash, S: BuildHasher, const N: usize> Len for IndexSet<K, S, N> {
    #[inline(always)]
    fn len(&self) -> usize {
        IndexSet::len(self)
    }
}

pub struct MapIter<'a, K>(heapless::index_set::Iter<'a, K>);

impl<'a, K> Iterator for MapIter<'a, K> {
    type Item = (&'a K, &'a ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|key| (key, &()))
    }
}

impl<'a, K: Eq + Hash + 'a, S: BuildHasher, const N: usize> Values<'a> for IndexSet<K, S, N>
where
    S: 'a,
{
    type Values = ValuesFromKeyValuePairs<MapIter<'a, K>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(MapIter(IndexSet::iter(self)))
    }
}

impl<'a, K: Eq + Hash + 'a, S: BuildHasher, const N: usize> Iter<'a, &'a K> for IndexSet<K, S, N>
where
    S: 'a,
{
    type Iter = MapIter<'a, K>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        MapIter(IndexSet::iter(self))
    }
}
