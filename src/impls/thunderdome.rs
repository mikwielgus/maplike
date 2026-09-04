// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thunderdome::{Arena, Index};

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, Get, Insert, Len, Modify, Push, Put, Remove, Set, WithOne};

impl<V> Container for Arena<V> {
    type Value = V;
}

impl<V> Keyed for Arena<V> {
    type Key = Index;
}

impl<V> WithOne<V> for Arena<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut arena = Arena::new();
        Arena::insert(&mut arena, element);

        arena
    }
}

impl<V> Get<Index> for Arena<V> {
    #[inline(always)]
    fn get(&self, key: &Index) -> Option<&V> {
        Arena::get(self, *key)
    }
}

impl<V> Set<Index> for Arena<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: Index, value: V) -> Option<V> {
        Arena::insert_at(self, key, value)
    }
}

impl<V> Modify<Index> for Arena<V> {
    #[inline(always)]
    fn modify<F>(&mut self, key: &Index, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(*key).expect("no value under key"));
    }
}

impl<V> Insert<Index> for Arena<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, key: Index, value: V) -> Option<V> {
        Arena::insert_at(self, key, value)
    }
}

impl<V> Remove<Index> for Arena<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &Index) -> Option<V> {
        Arena::remove(self, *key)
    }
}

impl<V> Push<Index> for Arena<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> Index {
        Arena::insert(self, value)
    }
}

impl<V> Put<V> for Arena<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Arena::insert(self, value);

        None
    }
}

impl<V> Clear for Arena<V> {
    #[inline(always)]
    fn clear(&mut self) {
        Arena::clear(self);
    }
}

impl<V> Len for Arena<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        Arena::len(self)
    }
}

impl<'a, V: 'a> Values<'a> for Arena<V> {
    type Values = ValuesFromKeyValuePairs<thunderdome::iter::Iter<'a, V>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Arena::iter(self))
    }
}

impl<V> IntoValues for Arena<V> {
    type IntoValues = ValuesFromKeyValuePairs<thunderdome::iter::IntoIter<V>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, V: 'a> Iter<'a, Index> for Arena<V> {
    type Iter = thunderdome::iter::Iter<'a, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        Arena::iter(self)
    }
}

impl<V> IntoIter<Index> for Arena<V> {
    type IntoIter = thunderdome::iter::IntoIter<V>;

    #[inline(always)]
    fn into_iter(self) -> thunderdome::iter::IntoIter<V> {
        IntoIterator::into_iter(self)
    }
}
