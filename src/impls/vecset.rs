// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use vecmap::VecSet;

use crate::abc::{Container, Keyed};
use crate::ctors::{WithCapacity, WithOne};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, ContainsKey, Get, Insert, Len, Put, Remove, Set};

impl<K> Container for VecSet<K> {
    type Value = ();
}

impl<K> Keyed for VecSet<K> {
    type Key = K;
}

impl<K> WithCapacity for VecSet<K> {
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self {
        VecSet::with_capacity(capacity)
    }
}

impl<K: Eq> WithOne<K> for VecSet<K> {
    #[inline(always)]
    fn with_one(element: K) -> Self {
        let mut set = VecSet::new();
        VecSet::insert(&mut set, element);

        set
    }
}

impl<K: Eq, Q: Eq + ?Sized> ContainsKey<Q> for VecSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        VecSet::contains(self, key)
    }
}

impl<K: Eq, Q: Eq + ?Sized> Get<Q> for VecSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        VecSet::get(self, key).map(|_| &())
    }
}

impl<K: Eq> Set<K> for VecSet<K> {
    type Output = bool;

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) -> bool {
        VecSet::insert(self, key)
    }
}

impl<K: Eq> Insert<K> for VecSet<K> {
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) -> bool {
        VecSet::insert(self, key)
    }
}

impl<K: Eq, Q: Eq + ?Sized> Remove<Q> for VecSet<K>
where
    K: Borrow<Q>,
{
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        VecSet::remove(self, key).then_some(())
    }
}

impl<V: Eq> Put<V> for VecSet<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        VecSet::insert(self, value);

        None
    }
}

impl<K> Clear for VecSet<K> {
    #[inline(always)]
    fn clear(&mut self) {
        VecSet::clear(self);
    }
}

impl<K> Len for VecSet<K> {
    #[inline(always)]
    fn len(&self) -> usize {
        VecSet::len(self)
    }
}

impl<'a, K: 'a> Values<'a> for VecSet<K> {
    type Values = ValuesFromKeyValuePairs<MapIter<'a, K>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Iter::iter(self))
    }
}

impl<K> IntoValues for VecSet<K> {
    type IntoValues = ValuesFromKeyValuePairs<MapIntoIter<K>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIter::into_iter(self))
    }
}

pub struct MapIter<'a, K>(vecmap::set::Iter<'a, K>);

impl<'a, K> Iterator for MapIter<'a, K> {
    type Item = (&'a K, &'a ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|key| (key, &()))
    }
}

impl<'a, K: 'a> Iter<'a, &'a K> for VecSet<K> {
    type Iter = MapIter<'a, K>;

    #[inline(always)]
    fn iter(&'a self) -> MapIter<'a, K> {
        MapIter(VecSet::iter(self))
    }
}

pub struct MapIntoIter<K>(vecmap::set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K> IntoIter<K> for VecSet<K> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
