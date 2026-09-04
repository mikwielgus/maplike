// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::abc::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Len, Modify, Put, Remove, Set, WithOne};

impl<V> Container for Option<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for Option<V> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        Some(element)
    }
}

impl<V> Get<usize> for Option<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        if *index == 0 { self.as_ref() } else { None }
    }
}

impl<V> Set<usize> for Option<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) -> Option<V> {
        assert_eq!(index, 0);
        self.replace(value)
    }
}

impl<V> Modify<usize> for Option<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        assert_eq!(*index, 0);
        f(self.as_mut().expect("no value under key"));
    }
}

impl<V> Remove<usize> for Option<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        if *index == 0 { self.take() } else { None }
    }
}

impl<V> Put<V> for Option<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        self.replace(value)
    }
}

impl<V> Clear for Option<V> {
    #[inline(always)]
    fn clear(&mut self) {
        *self = None;
    }
}

impl<V> Len for Option<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.is_some().into()
    }
}

impl<'a, V: 'a> Values<'a> for Option<V> {
    type Values = core::option::Iter<'a, V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        Option::iter(self)
    }
}

impl<V> IntoValues for Option<V> {
    type IntoValues = core::option::IntoIter<V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, V: 'a> Iter<'a, usize> for Option<V> {
    type Iter = core::iter::Enumerate<core::option::Iter<'a, V>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        Option::iter(self).enumerate()
    }
}

impl<V> IntoIter<usize> for Option<V> {
    type IntoIter = core::iter::Enumerate<core::option::IntoIter<V>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
