// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use bidimap::{BiBTreeMap, Overwritten};

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{
    Clear, Get, GetByLeft, GetByRight, Insert, Len, RemoveByLeft, RemoveByRight, Set,
};

impl<L, R> Container for BiBTreeMap<L, R> {
    type Value = R;
}

impl<L, R> Keyed for BiBTreeMap<L, R> {
    type Key = L;
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> Get<Q> for BiBTreeMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&R> {
        BiBTreeMap::get_by_left(self, key)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> GetByLeft<Q> for BiBTreeMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_left(&self, key: &Q) -> Option<&R> {
        BiBTreeMap::get_by_left(self, key)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> GetByRight<L, Q> for BiBTreeMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_right(&self, key: &Q) -> Option<&L> {
        BiBTreeMap::get_by_right(self, key)
    }
}

impl<L: Ord, R: Ord> Set<L> for BiBTreeMap<L, R> {
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn set(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiBTreeMap::insert(self, key, value)
    }
}

impl<L: Ord, R: Ord> Insert<L> for BiBTreeMap<L, R> {
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn insert(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiBTreeMap::insert(self, key, value)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> RemoveByLeft<Q> for BiBTreeMap<L, R>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_left(&mut self, key: &Q) -> Option<R> {
        BiBTreeMap::remove_by_left(self, key).map(|(_, value)| value)
    }
}

impl<L: Ord, R: Ord, Q: Ord + ?Sized> RemoveByRight<L, Q> for BiBTreeMap<L, R>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_right(&mut self, key: &Q) -> Option<L> {
        BiBTreeMap::remove_by_right(self, key).map(|(key, _)| key)
    }
}

impl<L: Ord, R: Ord> Clear for BiBTreeMap<L, R> {
    #[inline(always)]
    fn clear(&mut self) {
        BiBTreeMap::clear(self);
    }
}

impl<L, R> Len for BiBTreeMap<L, R> {
    #[inline(always)]
    fn len(&self) -> usize {
        BiBTreeMap::len(self)
    }
}

impl<'a, L: 'a, R: 'a> Values<'a> for BiBTreeMap<L, R> {
    type Values = ValuesFromKeyValuePairs<bidimap::btree::Iter<'a, L, R>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(BiBTreeMap::iter(self))
    }
}

impl<L, R> IntoValues for BiBTreeMap<L, R> {
    type IntoValues = ValuesFromKeyValuePairs<bidimap::btree::IntoIter<L, R>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, L: 'a, R: 'a> Iter<'a, &'a L> for BiBTreeMap<L, R> {
    type Iter = bidimap::btree::Iter<'a, L, R>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        BiBTreeMap::iter(self)
    }
}

impl<L, R> IntoIter<L> for BiBTreeMap<L, R> {
    type IntoIter = bidimap::btree::IntoIter<L, R>;

    #[inline(always)]
    fn into_iter(self) -> bidimap::btree::IntoIter<L, R> {
        IntoIterator::into_iter(self)
    }
}
