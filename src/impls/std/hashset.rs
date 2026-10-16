// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

use std_::collections::HashSet;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, ContainsKey, Get, Insert, Len, Put, Remove, Set, WithOne};

impl<K, S> Container for HashSet<K, S> {
    type Value = ();
}

impl<K, S> Keyed for HashSet<K, S> {
    type Key = K;
}

impl<K: Eq + Hash, S: BuildHasher + Default> WithOne<K> for HashSet<K, S> {
    #[inline(always)]
    fn with_one(element: K) -> Self {
        let mut hashset = HashSet::with_hasher(S::default());
        HashSet::insert(&mut hashset, element);

        hashset
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, S: BuildHasher> ContainsKey<Q> for HashSet<K, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        HashSet::contains(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, S: BuildHasher> Get<Q> for HashSet<K, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        HashSet::get(self, key).map(|_| &())
    }
}

impl<K: Eq + Hash, S: BuildHasher> Set<K> for HashSet<K, S> {
    type Output = bool;

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) -> bool {
        HashSet::insert(self, key)
    }
}

impl<K: Eq + Hash, S: BuildHasher> Insert<K> for HashSet<K, S> {
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) -> bool {
        HashSet::insert(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, S: BuildHasher> Remove<Q> for HashSet<K, S>
where
    K: Borrow<Q>,
{
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        HashSet::remove(self, key).then_some(())
    }
}

impl<V: Eq + Hash, S: BuildHasher> Put<V> for HashSet<V, S> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        HashSet::insert(self, value);

        None
    }
}

impl<K, S: BuildHasher> Clear for HashSet<K, S> {
    #[inline(always)]
    fn clear(&mut self) {
        HashSet::clear(self);
    }
}

impl<K, S> Len for HashSet<K, S> {
    #[inline(always)]
    fn len(&self) -> usize {
        HashSet::len(self)
    }
}

impl<'a, K: 'a, S: 'a> Values<'a> for HashSet<K, S> {
    type Values = ValuesFromKeyValuePairs<MapIter<'a, K>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Iter::iter(self))
    }
}

impl<K, S> IntoValues for HashSet<K, S> {
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

impl<'a, K: 'a, S: 'a> Iter<'a, &'a K> for HashSet<K, S> {
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

impl<K, S> IntoIter<K> for HashSet<K, S> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
