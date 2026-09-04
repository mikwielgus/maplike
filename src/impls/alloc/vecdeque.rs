// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::collections::VecDeque;

use crate::abc::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Len, Modify, Pop, Push, Put, Resize, Set, WithOne};

impl<V> Container for VecDeque<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for VecDeque<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut vecdeque = VecDeque::new();
        VecDeque::push_back(&mut vecdeque, element);

        vecdeque
    }
}

impl<V> Get<usize> for VecDeque<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        self.get(*index)
    }
}

impl<V> Set<usize> for VecDeque<V> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V> Modify<usize> for VecDeque<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(&mut self[*index]);
    }
}

impl<V> Push<usize> for VecDeque<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        VecDeque::push_back(self, value);

        self.len() - 1
    }
}

impl<V> Pop for VecDeque<V> {
    #[inline(always)]
    fn pop(&mut self) -> Option<V> {
        VecDeque::pop_back(self)
    }
}

impl<V> Put<V> for VecDeque<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        VecDeque::push_back(self, value);

        None
    }
}

impl<V> Clear for VecDeque<V> {
    #[inline(always)]
    fn clear(&mut self) {
        VecDeque::clear(self);
    }
}

impl<V> Len for VecDeque<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        VecDeque::len(self)
    }
}

impl<V> Resize for VecDeque<V> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: V)
    where
        V: Clone,
    {
        VecDeque::resize(self, new_len, value);
    }
}

impl<'a, V: 'a> Values<'a> for VecDeque<V> {
    type Values = alloc_::collections::vec_deque::Iter<'a, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        VecDeque::iter(self)
    }
}

impl<V> IntoValues for VecDeque<V> {
    type IntoValues = alloc_::collections::vec_deque::IntoIter<V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, V: 'a> Iter<'a, usize> for VecDeque<V> {
    type Iter = core::iter::Enumerate<alloc_::collections::vec_deque::Iter<'a, V>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        VecDeque::iter(self).enumerate()
    }
}

impl<V> IntoIter<usize> for VecDeque<V> {
    type IntoIter = core::iter::Enumerate<alloc_::collections::vec_deque::IntoIter<V>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
