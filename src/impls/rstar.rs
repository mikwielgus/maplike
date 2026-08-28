// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use rstar::{RTree, RTreeObject, RTreeParams};

use crate::containers::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Assign, Clear, Get, Insert, Len, Put, Remove, Set, WithOne};

impl<K: RTreeObject, Params: RTreeParams> Container for RTree<K, Params> {
    type Key = K;
    type Value = ();
}

impl<K: RTreeObject, Params: RTreeParams> WithOne<K> for RTree<K, Params> {
    #[inline(always)]
    fn with_one(element: K) -> Self {
        let mut rtree = RTree::new_with_params();
        RTree::insert(&mut rtree, element);

        rtree
    }
}

impl<K: RTreeObject, Params: RTreeParams> Assign for RTree<K, Params> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<K: RTreeObject + PartialEq, Params: RTreeParams> Get<K> for RTree<K, Params> {
    #[inline(always)]
    fn get(&self, key: &K) -> Option<&()> {
        RTree::contains(self, key).then_some(&())
    }
}

impl<K: RTreeObject + PartialEq, Params: RTreeParams> Set<K> for RTree<K, Params> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, key: K, _value: ()) {
        RTree::remove(self, &key);
        RTree::insert(self, key);
    }
}

impl<K: RTreeObject, Params: RTreeParams> Insert<K> for RTree<K, Params> {
    type Output = ();

    #[inline(always)]
    fn insert(&mut self, key: K, _value: ()) {
        RTree::insert(self, key);
    }
}

impl<K: RTreeObject + PartialEq, Params: RTreeParams> Remove<K> for RTree<K, Params> {
    type Output = Option<()>;

    #[inline(always)]
    fn remove(&mut self, key: &K) -> Option<()> {
        RTree::remove(self, key).map(|_| ())
    }
}

impl<K: RTreeObject, Params: RTreeParams> Put<K> for RTree<K, Params> {
    #[inline(always)]
    fn put(&mut self, key: K) -> Option<K> {
        RTree::insert(self, key);

        None
    }
}

impl<K: RTreeObject + PartialEq, Params: RTreeParams> Clear for RTree<K, Params> {
    #[inline(always)]
    fn clear(&mut self) {
        // TODO: Send a path upstream to implement `.clear()` efficiently,
        // without having to drain.
        self.drain().for_each(drop);
    }
}

impl<K: RTreeObject> Len for RTree<K> {
    #[inline(always)]
    fn len(&self) -> usize {
        RTree::size(self)
    }
}

impl<'a, K: RTreeObject + 'a, Params: RTreeParams + 'a> Values<'a> for RTree<K, Params> {
    type Values = ValuesFromKeyValuePairs<MapIter<'a, K>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Iter::iter(self))
    }
}

impl<K: RTreeObject, Params: RTreeParams> IntoValues for RTree<K, Params> {
    type IntoValues = ValuesFromKeyValuePairs<MapIntoIter<K>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIter::into_iter(self))
    }
}

pub struct MapIter<'a, K: RTreeObject>(rstar::iterators::RTreeIterator<'a, K>);

impl<'a, K: RTreeObject> Iterator for MapIter<'a, K> {
    type Item = (&'a K, &'a ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|key| (key, &()))
    }
}

impl<'a, K: RTreeObject + 'a, Params: RTreeParams + 'a> Iter<'a, &'a K> for RTree<K, Params> {
    type Iter = MapIter<'a, K>;

    #[inline(always)]
    fn iter(&'a self) -> MapIter<'a, K> {
        MapIter(RTree::iter(self))
    }
}

pub struct MapIntoIter<K: RTreeObject>(rstar::iterators::IntoIter<K>);

impl<K: RTreeObject> Iterator for MapIntoIter<K> {
    type Item = (K, ());

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|k| (k, ()))
    }
}

impl<K: RTreeObject, Params: RTreeParams> IntoIter<K> for RTree<K, Params> {
    type IntoIter = MapIntoIter<K>;

    #[inline(always)]
    fn into_iter(self) -> MapIntoIter<K> {
        MapIntoIter(IntoIterator::into_iter(self))
    }
}
