// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use alloc_::collections::BTreeSet;

use crate::containers::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Assign, Clear, Get, Insert, Len, Put, Remove, Set, WithOne};

impl<K> Container for BTreeSet<K> {
    type Key = K;
    type Value = ();
}

impl<K: Ord> WithOne<K> for BTreeSet<K> {
    #[inline(always)]
    fn with_one(element: K) -> Self {
        let mut btreeset = BTreeSet::new();
        BTreeSet::insert(&mut btreeset, element);

        btreeset
    }
}

impl<K> Assign for BTreeSet<K> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: Ord, Q: Ord + ?Sized> Get<Q> for BTreeSet<K>
where
    K: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&()> {
        BTreeSet::get(self, key).map(|_| &())
    }
}

impl<K: Ord> Set<K> for BTreeSet<K> {
    type Output = bool;

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) -> bool {
        BTreeSet::insert(self, key)
    }
}

impl<K: Ord> Insert<K> for BTreeSet<K> {
    type Output = bool;

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) -> bool {
        BTreeSet::insert(self, key)
    }
}

impl<K: Ord, Q: Ord + ?Sized> Remove<Q> for BTreeSet<K>
where
    K: Borrow<Q>,
{
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &Q) -> Option<()> {
        BTreeSet::remove(self, key).then_some(())
    }
}

impl<V: Ord> Put<V> for BTreeSet<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        BTreeSet::insert(self, value);

        None
    }
}

impl<K: Ord> Clear for BTreeSet<K> {
    #[inline(always)]
    fn clear(&mut self) {
        BTreeSet::clear(self);
    }
}

impl<K> Len for BTreeSet<K> {
    #[inline(always)]
    fn len(&self) -> usize {
        BTreeSet::len(self)
    }
}

impl<'a, K: 'a> Values<'a> for BTreeSet<K> {
    type Values = ValuesFromKeyValuePairs<MapIter<'a, K>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Iter::iter(self))
    }
}

impl<K> IntoValues for BTreeSet<K> {
    type IntoValues = ValuesFromKeyValuePairs<MapIntoIter<K>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIter::into_iter(self))
    }
}

pub struct MapIter<'a, K>(alloc_::collections::btree_set::Iter<'a, K>);

impl<'a, K> Iterator for MapIter<'a, K> {
    type Item = (&'a K, &'a ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|key| (key, &()))
    }
}

impl<'a, K: 'a> Iter<'a, &'a K> for BTreeSet<K> {
    type Iter = MapIter<'a, K>;

    #[inline(always)]
    fn iter(&'a self) -> MapIter<'a, K> {
        MapIter(BTreeSet::iter(self))
    }
}

pub struct MapIntoIter<K>(alloc_::collections::btree_set::IntoIter<K>);

impl<K> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K> IntoIter<K> for BTreeSet<K> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
