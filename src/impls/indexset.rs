// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::{BuildHasher, Hash};

use indexmap::IndexSet;

use crate::abc::{Container, Keyed};
use crate::ctors::{WithCapacity, WithCapacityAndHasher, WithHasher, WithOne};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, ContainsKey, Get, Insert, Len, Put, Remove, Set};

impl<K, S> Container for IndexSet<K, S> {
    type Value = ();
}

impl<K, S> Keyed for IndexSet<K, S> {
    type Key = K;
}

impl<K, S: BuildHasher + Default> WithCapacity for IndexSet<K, S> {
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self {
        IndexSet::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K, S> WithHasher<S> for IndexSet<K, S> {
    #[inline(always)]
    fn with_hasher(hasher: S) -> Self {
        IndexSet::with_hasher(hasher)
    }
}

impl<K, S> WithCapacityAndHasher<S> for IndexSet<K, S> {
    #[inline(always)]
    fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        IndexSet::with_capacity_and_hasher(capacity, hasher)
    }
}

impl<K: Eq + Hash, S: BuildHasher + Default> WithOne<K> for IndexSet<K, S> {
    #[inline(always)]
    fn with_one(element: K) -> Self {
        let mut indexset = IndexSet::with_hasher(S::default());
        IndexSet::insert(&mut indexset, element);

        indexset
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, S: BuildHasher> ContainsKey<Q> for IndexSet<K, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        IndexSet::contains(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, S: BuildHasher> Get<Q> for IndexSet<K, S>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        IndexSet::get(self, key).map(|_| &())
    }
}

impl<K: Eq + Hash, S: BuildHasher> Set<K> for IndexSet<K, S> {
    type Output = bool;

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) -> bool {
        IndexSet::insert(self, key)
    }
}

impl<K: Eq + Hash, S: BuildHasher> Insert<K> for IndexSet<K, S> {
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) -> bool {
        IndexSet::insert(self, key)
    }
}

impl<K: Eq + Hash, Q: Eq + Hash + ?Sized, S: BuildHasher> Remove<Q> for IndexSet<K, S>
where
    K: Borrow<Q>,
{
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        IndexSet::shift_remove(self, key).then_some(())
    }
}

impl<V: Eq + Hash, S: BuildHasher> Put<V> for IndexSet<V, S> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        IndexSet::insert(self, value);

        None
    }
}

impl<K: Eq + Hash, S: BuildHasher> Clear for IndexSet<K, S> {
    #[inline(always)]
    fn clear(&mut self) {
        IndexSet::clear(self);
    }
}

impl<K, S> Len for IndexSet<K, S> {
    #[inline(always)]
    fn len(&self) -> usize {
        IndexSet::len(self)
    }
}

impl<'a, K: 'a, S: 'a> Values<'a> for IndexSet<K, S> {
    type Values = ValuesFromKeyValuePairs<MapIter<'a, K>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Iter::iter(self))
    }
}

impl<K, S> IntoValues for IndexSet<K, S> {
    type IntoValues = ValuesFromKeyValuePairs<MapIntoIter<K>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIter::into_iter(self))
    }
}

pub struct MapIter<'a, K>(indexmap::set::Iter<'a, K>);

impl<'a, K> Iterator for MapIter<'a, K> {
    type Item = (&'a K, &'a ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|key| (key, &()))
    }
}

impl<'a, K: 'a, S: 'a> Iter<'a, &'a K> for IndexSet<K, S> {
    type Iter = MapIter<'a, K>;

    #[inline(always)]
    fn iter(&'a self) -> MapIter<'a, K> {
        MapIter(IndexSet::iter(self))
    }
}

pub struct MapIntoIter<K>(indexmap::set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K, S> IntoIter<K> for IndexSet<K, S> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
