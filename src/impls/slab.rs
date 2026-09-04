// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use slab::Slab;

use crate::abc::Keyed;
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, Get, Len, Modify, Push, Put, Remove, Set, WithOne};

impl<V> Keyed for Slab<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for Slab<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut slab = Slab::new();
        Slab::insert(&mut slab, element);

        slab
    }
}

impl<V> Get<usize> for Slab<V> {
    #[inline(always)]
    fn get(&self, key: &usize) -> Option<&V> {
        Slab::get(self, *key)
    }
}

impl<V> Set<usize> for Slab<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, key: usize, value: V) -> Option<V> {
        Some(core::mem::replace(&mut self[key], value))
    }
}

impl<V> Modify<usize> for Slab<V> {
    #[inline(always)]
    fn modify<F>(&mut self, key: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(*key).expect("no value under key"));
    }
}

impl<V> Remove<usize> for Slab<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, key: &usize) -> Option<V> {
        Slab::try_remove(self, *key)
    }
}

impl<V> Push<usize> for Slab<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        Slab::insert(self, value)
    }
}

impl<V> Put<V> for Slab<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Slab::insert(self, value);

        None
    }
}

impl<V> Clear for Slab<V> {
    #[inline(always)]
    fn clear(&mut self) {
        Slab::clear(self);
    }
}

impl<V> Len for Slab<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        Slab::len(self)
    }
}

impl<'a, V: 'a> Values<'a> for Slab<V> {
    type Values = ValuesFromKeyValuePairs<slab::Iter<'a, V>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(Slab::iter(self))
    }
}

impl<V> IntoValues for Slab<V> {
    type IntoValues = ValuesFromKeyValuePairs<slab::IntoIter<V>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, V: 'a> Iter<'a, usize> for Slab<V> {
    type Iter = slab::Iter<'a, V>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        Slab::iter(self)
    }
}

impl<V> IntoIter<usize> for Slab<V> {
    type IntoIter = slab::IntoIter<V>;

    #[inline(always)]
    fn into_iter(self) -> slab::IntoIter<V> {
        IntoIterator::into_iter(self)
    }
}
