// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::vec::Vec;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Len, Modify, Pop, Push, Put, Resize, Set, SwapRemove, WithOne};

impl<V> Container for Vec<V> {
    type Value = V;
}

impl<V> Keyed for Vec<V> {
    type Key = usize;
}

impl<V> WithOne<V> for Vec<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut vec = Vec::new();
        Vec::push(&mut vec, element);

        vec
    }
}

impl<V> Get<usize> for Vec<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        self.as_slice().get(*index)
    }
}

impl<V> Set<usize> for Vec<V> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V> Modify<usize> for Vec<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(&mut self[*index]);
    }
}

impl<V> Push<usize> for Vec<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        Vec::push(self, value);

        self.len() - 1
    }
}

impl<V> Pop for Vec<V> {
    #[inline(always)]
    fn pop(&mut self) -> Option<V> {
        Vec::pop(self)
    }
}

impl<V> SwapRemove<usize> for Vec<V> {
    type Output = V;

    #[inline(always)]
    fn swap_remove(&mut self, key: &usize) -> V {
        Vec::swap_remove(self, *key)
    }
}

impl<V> Put<V> for Vec<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Vec::push(self, value);

        None
    }
}

impl<V> Clear for Vec<V> {
    #[inline(always)]
    fn clear(&mut self) {
        Vec::clear(self);
    }
}

impl<V> Len for Vec<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl<V> Resize for Vec<V> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: V)
    where
        V: Clone,
    {
        Vec::resize(self, new_len, value);
    }
}

impl<'a, V: 'a> Values<'a> for Vec<V> {
    type Values = core::slice::Iter<'a, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        self.as_slice().iter()
    }
}

impl<V> IntoValues for Vec<V> {
    type IntoValues = alloc_::vec::IntoIter<V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, V: 'a> Iter<'a, usize> for Vec<V> {
    type Iter = core::iter::Enumerate<core::slice::Iter<'a, V>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.as_slice().iter().enumerate()
    }
}

impl<V> IntoIter<usize> for Vec<V> {
    type IntoIter = core::iter::Enumerate<alloc_::vec::IntoIter<V>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
