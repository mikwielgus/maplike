// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use vecmap::VecMap;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, ContainsKey, Get, Insert, Len, Modify, Remove, Set, WithCapacity};

impl<K, V> Container for VecMap<K, V> {
    type Value = V;
}

impl<K, V> Keyed for VecMap<K, V> {
    type Key = K;
}

impl<K, V> WithCapacity for VecMap<K, V> {
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self {
        VecMap::with_capacity(capacity)
    }
}

impl<K: Eq, Q: Eq + ?Sized, V> ContainsKey<Q> for VecMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        VecMap::contains_key(self, key)
    }
}

impl<K: Eq, Q: Eq + ?Sized, V> Get<Q> for VecMap<K, V>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&V> {
        VecMap::get(self, key)
    }
}

impl<K: Eq, V> Set<K> for VecMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: K, value: V) -> Option<V> {
        VecMap::insert(self, key, value)
    }
}

impl<K: Eq, Q: Eq + ?Sized, V> Modify<Q> for VecMap<K, V>
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

impl<K: Eq, V> Insert<K> for VecMap<K, V> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        VecMap::insert(self, key, value)
    }
}

impl<K: Eq, Q: Eq + ?Sized, V> Remove<Q> for VecMap<K, V>
where
    K: Borrow<Q>,
{
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<V> {
        VecMap::remove(self, key)
    }
}

impl<K, V> Clear for VecMap<K, V> {
    #[inline(always)]
    fn clear(&mut self) {
        VecMap::clear(self);
    }
}

impl<K, V> Len for VecMap<K, V> {
    #[inline(always)]
    fn len(&self) -> usize {
        VecMap::len(self)
    }
}

impl<'a, K: 'a, V: 'a> Values<'a> for VecMap<K, V> {
    type Values = vecmap::map::Values<'a, K, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        VecMap::values(self)
    }
}

impl<K, V> IntoValues for VecMap<K, V> {
    type IntoValues = vecmap::map::IntoValues<K, V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        VecMap::into_values(self)
    }
}

impl<'a, K: 'a, V: 'a> Iter<'a, &'a K> for VecMap<K, V> {
    type Iter = vecmap::map::Iter<'a, K, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        VecMap::iter(self)
    }
}

impl<K, V> IntoIter<K> for VecMap<K, V> {
    type IntoIter = vecmap::map::IntoIter<K, V>;

    #[inline(always)]
    fn into_iter(self) -> vecmap::map::IntoIter<K, V> {
        IntoIterator::into_iter(self)
    }
}
