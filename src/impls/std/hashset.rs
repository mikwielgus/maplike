// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use std_::{collections::HashSet, hash::Hash};

use crate::containers::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, Get, Insert, Len, Put, Remove, Set, WithOne};

impl<K> Container for HashSet<K> {
    type Key = K;
    type Value = ();
}

impl<K: Eq + Hash> WithOne<K> for HashSet<K> {
    #[inline(always)]
    fn with_one(element: K) -> Self {
        let mut hashset = HashSet::new();
        HashSet::insert(&mut hashset, element);

        hashset
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized> Get<Q> for HashSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        HashSet::get(self, key).map(|_| &())
    }
}

impl<K: Eq + Hash> Set<K> for HashSet<K> {
    type Output = bool;

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) -> bool {
        HashSet::insert(self, key)
    }
}

impl<K: Eq + Hash> Insert<K> for HashSet<K> {
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) -> bool {
        HashSet::insert(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized> Remove<Q> for HashSet<K>
where
    K: Borrow<Q>,
{
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        HashSet::remove(self, key).then_some(())
    }
}

impl<V: Eq + Hash> Put<V> for HashSet<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        HashSet::insert(self, value);

        None
    }
}

impl<K: Eq + Hash> Clear for HashSet<K> {
    #[inline(always)]
    fn clear(&mut self) {
        HashSet::clear(self);
    }
}

impl<K> Len for HashSet<K> {
    #[inline(always)]
    fn len(&self) -> usize {
        HashSet::len(self)
    }
}

impl<'a, K: 'a> Values<'a> for HashSet<K> {
    type Values = ValuesFromKeyValuePairs<MapIter<'a, K>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Iter::iter(self))
    }
}

impl<K> IntoValues for HashSet<K> {
    type IntoValues = ValuesFromKeyValuePairs<MapIntoIter<K>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIter::into_iter(self))
    }
}

pub struct MapIter<'a, K>(std_::collections::hash_set::Iter<'a, K>);

impl<'a, K> Iterator for MapIter<'a, K> {
    type Item = (&'a K, &'a ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|key| (key, &()))
    }
}

impl<'a, K: 'a> Iter<'a, &'a K> for HashSet<K> {
    type Iter = MapIter<'a, K>;

    #[inline(always)]
    fn iter(&'a self) -> MapIter<'a, K> {
        MapIter(HashSet::iter(self))
    }
}

pub struct MapIntoIter<K>(std_::collections::hash_set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K> IntoIter<K> for HashSet<K> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
