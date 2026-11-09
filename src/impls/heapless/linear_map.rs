// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use heapless::LinearMap;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{CheckedInsert, Clear, ContainsKey, Get, Len, Modify, Remove, Set};

impl<K: Eq, V, const N: usize> Container for LinearMap<K, V, N> {
    type Value = V;
}

impl<K: Eq, V, const N: usize> Keyed for LinearMap<K, V, N> {
    type Key = K;
}

impl<K: Eq, Q: Eq + ?Sized, V, const N: usize> ContainsKey<Q> for LinearMap<K, V, N>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        LinearMap::get(self, key).is_some()
    }
}

impl<K: Eq, Q: Eq + ?Sized, V, const N: usize> Get<Q> for LinearMap<K, V, N>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        LinearMap::get(self, key)
    }
}

impl<K: Eq, V, const N: usize> Set<K> for LinearMap<K, V, N> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        let slot = LinearMap::get_mut(self, &key).expect("no value under key");

        Some(core::mem::replace(slot, value))
    }
}

impl<K: Eq, Q: Eq + ?Sized, V, const N: usize> Modify<Q> for LinearMap<K, V, N>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn modify<F>(&mut self, key: &Q, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(LinearMap::get_mut(self, key).expect("no value under key"));
    }
}

impl<K: Eq, V, const N: usize> CheckedInsert<K> for LinearMap<K, V, N> {
    type Output = Option<V>;

    #[inline(always)]
    fn checked_insert(&mut self, key: K, value: V) -> Option<Self::Output> {
        match LinearMap::insert(self, key, value) {
            Ok(output) => Some(output),
            Err(_) => None,
        }
    }
}

impl<K: Eq, Q: Eq + ?Sized, V, const N: usize> Remove<Q> for LinearMap<K, V, N>
where
    K: Borrow<Q>,
{
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        LinearMap::remove(self, key)
    }
}

impl<K: Eq, V, const N: usize> Clear for LinearMap<K, V, N> {
    #[inline(always)]
    fn clear(&mut self) {
        LinearMap::clear(self);
    }
}

impl<K: Eq, V, const N: usize> Len for LinearMap<K, V, N> {
    #[inline(always)]
    fn len(&self) -> usize {
        LinearMap::len(self)
    }
}

impl<'a, K: Eq + 'a, V: 'a, const N: usize> Values<'a> for LinearMap<K, V, N> {
    type Values = HeaplessLinearMapValues<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        HeaplessLinearMapValues {
            inner: LinearMap::iter(self),
        }
    }
}

pub struct HeaplessLinearMapValues<'a, K, V> {
    inner: heapless::linear_map::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for HeaplessLinearMapValues<'a, K, V> {
    type Item = &'a V;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a V> {
        self.inner.next().map(|(_, value)| value)
    }
}

impl<K: Eq, V, const N: usize> IntoValues for LinearMap<K, V, N> {
    type IntoValues = ValuesFromKeyValuePairs<heapless::linear_map::IntoIter<K, V, N>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, K: Eq + 'a, V: 'a, const N: usize> Iter<'a, &'a K> for LinearMap<K, V, N> {
    type Iter = heapless::linear_map::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        LinearMap::iter(self)
    }
}

impl<K: Eq, V, const N: usize> IntoIter<K> for LinearMap<K, V, N> {
    type IntoIter = heapless::linear_map::IntoIter<K, V, N>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self)
    }
}
