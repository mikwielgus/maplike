// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use alloc_::collections::LinkedList;

use crate::abc::{Container, Keyed};
use crate::ctors::WithOne;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, ContainsKey, Get, Len, Modify, Pop, Push, Put, Set};

impl<V> Container for LinkedList<V> {
    type Value = V;
}

impl<V> Keyed for LinkedList<V> {
    type Key = usize;
}

impl<V> WithOne<V> for LinkedList<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut list = LinkedList::new();
        LinkedList::push_back(&mut list, element);

        list
    }
}

impl<V> ContainsKey<usize> for LinkedList<V> {
    #[inline(always)]
    fn contains_key(&self, index: &usize) -> bool {
        *index < self.len()
    }
}

impl<V> Get<usize> for LinkedList<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        self.iter().nth(*index)
    }
}

impl<V> Set<usize> for LinkedList<V> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        *self.iter_mut().nth(index).unwrap() = value;
    }
}

impl<V> Modify<usize> for LinkedList<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.iter_mut().nth(*index).unwrap());
    }
}

impl<V> Push<usize> for LinkedList<V> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        LinkedList::push_back(self, value);

        self.len() - 1
    }
}

impl<V> Pop for LinkedList<V> {
    #[inline(always)]
    fn pop(&mut self) -> Option<V> {
        LinkedList::pop_back(self)
    }
}

impl<V> Put<V> for LinkedList<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        LinkedList::push_back(self, value);

        None
    }
}

impl<V> Clear for LinkedList<V> {
    #[inline(always)]
    fn clear(&mut self) {
        LinkedList::clear(self);
    }
}

impl<V> Len for LinkedList<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        LinkedList::len(self)
    }
}

impl<'a, V: 'a> Values<'a> for LinkedList<V> {
    type Values = alloc_::collections::linked_list::Iter<'a, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        LinkedList::iter(self)
    }
}

impl<V> IntoValues for LinkedList<V> {
    type IntoValues = alloc_::collections::linked_list::IntoIter<V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, V: 'a> Iter<'a, usize> for LinkedList<V> {
    type Iter = core::iter::Enumerate<alloc_::collections::linked_list::Iter<'a, V>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        LinkedList::iter(self).enumerate()
    }
}

impl<V> IntoIter<usize> for LinkedList<V> {
    type IntoIter = core::iter::Enumerate<alloc_::collections::linked_list::IntoIter<V>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
