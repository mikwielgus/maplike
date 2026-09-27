// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use slotmap::{Key, SecondaryMap, SlotMap};

#[cfg(feature = "std")]
use slotmap::SparseSecondaryMap;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, ContainsKey, Get, Insert, Len, Modify, Push, Put, Remove, Set, WithOne};

impl<K: Key, V> Container for SlotMap<K, V> {
    type Value = V;
}

impl<K: Key, V> Keyed for SlotMap<K, V> {
    type Key = K;
}

impl<K: Key, V> WithOne<V> for SlotMap<K, V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut slot_map = SlotMap::with_key();
        SlotMap::insert(&mut slot_map, element);

        slot_map
    }
}

impl<K: Key, V> ContainsKey<K> for SlotMap<K, V> {
    #[inline(always)]
    fn contains_key(&self, key: &K) -> bool {
        SlotMap::contains_key(self, *key)
    }
}

impl<K: Key, V> Get<K> for SlotMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        SlotMap::get(self, *key)
    }
}

impl<K: Key, V> Set<K> for SlotMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        Some(core::mem::replace(&mut self[key], value))
    }
}

impl<K: Key, V> Modify<K> for SlotMap<K, V> {
    #[inline(always)]
    fn modify<F>(&mut self, key: &K, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(*key).expect("no value under key"));
    }
}

impl<K: Key, V> Remove<K> for SlotMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        SlotMap::remove(self, *key)
    }
}

impl<K: Key, V> Push<K> for SlotMap<K, V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> K {
        SlotMap::insert(self, value)
    }
}

impl<K: Key, V> Put<V> for SlotMap<K, V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        SlotMap::insert(self, value);

        None
    }
}

impl<K: Key, V> Clear for SlotMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        SlotMap::clear(self);
    }
}

impl<K: Key, V> Len for SlotMap<K, V> {
    #[inline(always)]
    fn len(&self) -> usize {
        SlotMap::len(self)
    }
}

impl<'a, K: Key + 'a, V: 'a> Values<'a> for SlotMap<K, V> {
    type Values = slotmap::basic::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        SlotMap::values(self)
    }
}

impl<K: Key, V> IntoValues for SlotMap<K, V> {
    type IntoValues = ValuesFromKeyValuePairs<slotmap::basic::IntoIter<K, V>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, K: Key + 'a, V: 'a> Iter<'a, K> for SlotMap<K, V> {
    type Iter = slotmap::basic::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        SlotMap::iter(self)
    }
}

impl<K: Key, V> IntoIter<K> for SlotMap<K, V> {
    type IntoIter = slotmap::basic::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

impl<K: Key, V> Container for SecondaryMap<K, V> {
    type Value = V;
}

impl<K: Key, V> Keyed for SecondaryMap<K, V> {
    type Key = K;
}

impl<K: Key, V> ContainsKey<K> for SecondaryMap<K, V> {
    #[inline(always)]
    fn contains_key(&self, key: &K) -> bool {
        SecondaryMap::contains_key(self, *key)
    }
}

impl<K: Key, V> Get<K> for SecondaryMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        SecondaryMap::get(self, *key)
    }
}

impl<K: Key, V> Set<K> for SecondaryMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        SecondaryMap::insert(self, key, value)
    }
}

impl<K: Key, V> Modify<K> for SecondaryMap<K, V> {
    #[inline(always)]
    fn modify<F>(&mut self, key: &K, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(*key).expect("no value under key"));
    }
}

impl<K: Key, V> Insert<K> for SecondaryMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        SecondaryMap::insert(self, key, value)
    }
}

impl<K: Key, V> Remove<K> for SecondaryMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        SecondaryMap::remove(self, *key)
    }
}

impl<K: Key, V> Clear for SecondaryMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        SecondaryMap::clear(self);
    }
}

impl<K: Key, V> Len for SecondaryMap<K, V> {
    #[inline(always)]
    fn len(&self) -> usize {
        SecondaryMap::len(self)
    }
}

impl<'a, K: Key + 'a, V: 'a> Values<'a> for SecondaryMap<K, V> {
    type Values = slotmap::secondary::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        SecondaryMap::values(self)
    }
}

impl<K: Key, V> IntoValues for SecondaryMap<K, V> {
    type IntoValues = ValuesFromKeyValuePairs<slotmap::secondary::IntoIter<K, V>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, K: Key + 'a, V: 'a> Iter<'a, K> for SecondaryMap<K, V> {
    type Iter = slotmap::secondary::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        SecondaryMap::iter(self)
    }
}

impl<K: Key, V> IntoIter<K> for SecondaryMap<K, V> {
    type IntoIter = slotmap::secondary::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Container for SparseSecondaryMap<K, V> {
    type Value = V;
}

#[cfg(feature = "std")]
impl<K: Key, V> Keyed for SparseSecondaryMap<K, V> {
    type Key = K;
}

#[cfg(feature = "std")]
impl<K: Key, V> ContainsKey<K> for SparseSecondaryMap<K, V> {
    #[inline(always)]
    fn contains_key(&self, key: &K) -> bool {
        SparseSecondaryMap::contains_key(self, *key)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Get<K> for SparseSecondaryMap<K, V> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&V> {
        SparseSecondaryMap::get(self, *key)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Set<K> for SparseSecondaryMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        SparseSecondaryMap::insert(self, key, value)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Modify<K> for SparseSecondaryMap<K, V> {
    #[inline(always)]
    fn modify<F>(&mut self, key: &K, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(*key).expect("no value under key"));
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Insert<K> for SparseSecondaryMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        SparseSecondaryMap::insert(self, key, value)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Remove<K> for SparseSecondaryMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<V> {
        SparseSecondaryMap::remove(self, *key)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Clear for SparseSecondaryMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        SparseSecondaryMap::clear(self);
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> Len for SparseSecondaryMap<K, V> {
    #[inline(always)]
    fn len(&self) -> usize {
        SparseSecondaryMap::len(self)
    }
}

#[cfg(feature = "std")]
impl<'a, K: Key + 'a, V: 'a> Values<'a> for SparseSecondaryMap<K, V> {
    type Values = slotmap::sparse_secondary::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        SparseSecondaryMap::values(self)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> IntoValues for SparseSecondaryMap<K, V> {
    type IntoValues = ValuesFromKeyValuePairs<slotmap::sparse_secondary::IntoIter<K, V>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

#[cfg(feature = "std")]
impl<'a, K: Key + 'a, V: 'a> Iter<'a, K> for SparseSecondaryMap<K, V> {
    type Iter = slotmap::sparse_secondary::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        SparseSecondaryMap::iter(self)
    }
}

#[cfg(feature = "std")]
impl<K: Key, V> IntoIter<K> for SparseSecondaryMap<K, V> {
    type IntoIter = slotmap::sparse_secondary::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}
