// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use bidimap::{BiHashMap, Overwritten};
use std_::hash::Hash;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{
    Clear, Get, GetByLeft, GetByRight, Insert, Len, RemoveByLeft, RemoveByRight, Set,
};

impl<L, R> Container for BiHashMap<L, R> {
    type Value = R;
}

impl<L, R> Keyed for BiHashMap<L, R> {
    type Key = L;
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> Get<Q> for BiHashMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&R> {
        BiHashMap::get_by_left(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> GetByLeft<Q> for BiHashMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_left(&self, key: &Q) -> Option<&R> {
        BiHashMap::get_by_left(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> GetByRight<L, Q> for BiHashMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_right(&self, key: &Q) -> Option<&L> {
        BiHashMap::get_by_right(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash> Set<L> for BiHashMap<L, R> {
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn set(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiHashMap::insert(self, key, value)
    }
}

impl<L: Eq + Hash, R: Eq + Hash> Insert<L> for BiHashMap<L, R> {
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn insert(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiHashMap::insert(self, key, value)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> RemoveByLeft<Q> for BiHashMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_left(&mut self, key: &Q) -> Option<R> {
        BiHashMap::remove_by_left(self, key).map(|(_, value)| value)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized> RemoveByRight<L, Q> for BiHashMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_right(&mut self, key: &Q) -> Option<L> {
        BiHashMap::remove_by_right(self, key).map(|(key, _)| key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash> Clear for BiHashMap<L, R> {
    #[inline(always)]
    fn clear(&mut self) {
        BiHashMap::clear(self);
    }
}

impl<L, R> Len for BiHashMap<L, R> {
    #[inline(always)]
    fn len(&self) -> usize {
        BiHashMap::len(self)
    }
}

impl<'a, L: 'a, R: 'a> Values<'a> for BiHashMap<L, R> {
    type Values = ValuesFromKeyValuePairs<bidimap::hash::Iter<'a, L, R>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(BiHashMap::iter(self))
    }
}

impl<L, R> IntoValues for BiHashMap<L, R> {
    type IntoValues = ValuesFromKeyValuePairs<bidimap::hash::IntoIter<L, R>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, L: 'a, R: 'a> Iter<'a, &'a L> for BiHashMap<L, R> {
    type Iter = bidimap::hash::Iter<'a, L, R>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        BiHashMap::iter(self)
    }
}

impl<L, R> IntoIter<L> for BiHashMap<L, R> {
    type IntoIter = bidimap::hash::IntoIter<L, R>;

    #[inline(always)]
    fn into_iter(self) -> bidimap::hash::IntoIter<L, R> {
        IntoIterator::into_iter(self)
    }
}
