// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use thin_vec::ThinVec;

use crate::abc::{Container, Keyed};
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Len, Modify, Pop, Push, Put, Resize, Set, SwapRemove, WithOne};

impl<V> Container for ThinVec<V> {
    type Value = V;
}

impl<V> Keyed for ThinVec<V> {
    type Key = usize;
}

impl<V> WithOne<V> for ThinVec<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut thin_vec = ThinVec::new();
        ThinVec::push(&mut thin_vec, element);

        thin_vec
    }
}

impl<V> Get<usize> for ThinVec<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        self.as_slice().get(*index)
    }
}

impl<V> Set<usize> for ThinVec<V> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V> Modify<usize> for ThinVec<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(&mut self[*index]);
    }
}

impl<V> Push<usize> for ThinVec<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        ThinVec::push(self, value);

        self.len() - 1
    }
}

impl<V> Pop for ThinVec<V> {
    #[inline(always)]
    fn pop(&mut self) -> Option<V> {
        ThinVec::pop(self)
    }
}

impl<V> SwapRemove<usize> for ThinVec<V> {
    type Output = V;

    #[inline(always)]
    fn swap_remove(&mut self, key: &usize) -> V {
        ThinVec::swap_remove(self, *key)
    }
}

impl<V> Put<V> for ThinVec<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        ThinVec::push(self, value);

        None
    }
}

impl<V> Clear for ThinVec<V> {
    #[inline(always)]
    fn clear(&mut self) {
        ThinVec::clear(self);
    }
}

impl<V> Len for ThinVec<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        ThinVec::len(self)
    }
}

impl<V> Resize for ThinVec<V> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: V)
    where
        V: Clone,
    {
        ThinVec::resize(self, new_len, value);
    }
}

impl<'a, V: 'a> Values<'a> for ThinVec<V> {
    type Values = core::slice::Iter<'a, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        self.as_slice().iter()
    }
}

impl<V> IntoValues for ThinVec<V> {
    type IntoValues = thin_vec::IntoIter<V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, V: 'a> Iter<'a, usize> for ThinVec<V> {
    type Iter = core::iter::Enumerate<core::slice::Iter<'a, V>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.as_slice().iter().enumerate()
    }
}

impl<V> IntoIter<usize> for ThinVec<V> {
    type IntoIter = core::iter::Enumerate<thin_vec::IntoIter<V>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
