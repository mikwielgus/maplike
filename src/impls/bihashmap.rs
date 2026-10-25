// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;
use core::hash::BuildHasher;

use bidimap::{BiHashMap, Overwritten};
use std_::hash::Hash;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{
    Clear, ContainsKey, Get, GetByLeft, GetByRight, Insert, Len, RemoveByLeft, RemoveByRight, Set,
    WithCapacity,
};

impl<L, R, LS, RS> Container for BiHashMap<L, R, LS, RS> {
    type Value = R;
}

impl<L, R, LS, RS> Keyed for BiHashMap<L, R, LS, RS> {
    type Key = L;
}

impl<L, R, LS: BuildHasher + Default, RS: BuildHasher + Default> WithCapacity
    for BiHashMap<L, R, LS, RS>
{
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self {
        BiHashMap::with_capacity_and_hashers(capacity, LS::default(), RS::default())
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized, LS: BuildHasher, RS: BuildHasher>
    ContainsKey<Q> for BiHashMap<L, R, LS, RS>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn contains_key(&self, key: &Q) -> bool {
        BiHashMap::get_by_left(self, key).is_some()
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized, LS: BuildHasher, RS: BuildHasher> Get<Q>
    for BiHashMap<L, R, LS, RS>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get(&self, key: &Q) -> Option<&R> {
        BiHashMap::get_by_left(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized, LS: BuildHasher, RS: BuildHasher>
    GetByLeft<Q> for BiHashMap<L, R, LS, RS>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_left(&self, key: &Q) -> Option<&R> {
        BiHashMap::get_by_left(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized, LS: BuildHasher, RS: BuildHasher>
    GetByRight<L, Q> for BiHashMap<L, R, LS, RS>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn get_by_right(&self, key: &Q) -> Option<&L> {
        BiHashMap::get_by_right(self, key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, LS: BuildHasher, RS: BuildHasher> Set<L>
    for BiHashMap<L, R, LS, RS>
{
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn set(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiHashMap::insert(self, key, value)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, LS: BuildHasher, RS: BuildHasher> Insert<L>
    for BiHashMap<L, R, LS, RS>
{
    type Output = Overwritten<L, R>;

    #[inline(always)]
    fn insert(&mut self, key: L, value: R) -> Overwritten<L, R> {
        BiHashMap::insert(self, key, value)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized, LS: BuildHasher, RS: BuildHasher>
    RemoveByLeft<Q> for BiHashMap<L, R, LS, RS>
where
    L: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_left(&mut self, key: &Q) -> Option<R> {
        BiHashMap::remove_by_left(self, key).map(|(_, value)| value)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, Q: Eq + Hash + ?Sized, LS: BuildHasher, RS: BuildHasher>
    RemoveByRight<L, Q> for BiHashMap<L, R, LS, RS>
where
    R: Borrow<Q>,
{
    #[inline(always)]
    fn remove_by_right(&mut self, key: &Q) -> Option<L> {
        BiHashMap::remove_by_right(self, key).map(|(key, _)| key)
    }
}

impl<L: Eq + Hash, R: Eq + Hash, LS: BuildHasher, RS: BuildHasher> Clear
    for BiHashMap<L, R, LS, RS>
{
    #[inline(always)]
    fn clear(&mut self) {
        BiHashMap::clear(self);
    }
}

impl<L, R, LS, RS> Len for BiHashMap<L, R, LS, RS> {
    #[inline(always)]
    fn len(&self) -> usize {
        BiHashMap::len(self)
    }
}

impl<'a, L: 'a, R: 'a, LS: 'a, RS: 'a> Values<'a> for BiHashMap<L, R, LS, RS> {
    type Values = ValuesFromKeyValuePairs<bidimap::hash::Iter<'a, L, R>>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        ValuesFromKeyValuePairs(BiHashMap::iter(self))
    }
}

impl<L, R, LS, RS> IntoValues for BiHashMap<L, R, LS, RS> {
    type IntoValues = ValuesFromKeyValuePairs<bidimap::hash::IntoIter<L, R>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, L: 'a, R: 'a, LS: 'a, RS: 'a> Iter<'a, &'a L> for BiHashMap<L, R, LS, RS> {
    type Iter = bidimap::hash::Iter<'a, L, R>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        BiHashMap::iter(self)
    }
}

impl<L, R, LS, RS> IntoIter<L> for BiHashMap<L, R, LS, RS> {
    type IntoIter = bidimap::hash::IntoIter<L, R>;

    #[inline(always)]
    fn into_iter(self) -> bidimap::hash::IntoIter<L, R> {
        IntoIterator::into_iter(self)
    }
}
