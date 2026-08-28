// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! `One`, a collection that holds always exactly one element.

use crate::containers::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Assign, Get, Len, Modify, Put, Set, WithOne};

/// A collection that holds exactly one element.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct One<V> {
    value: V,
}

impl<V> One<V> {
    /// Create a new always-single-valued collection.
    pub fn new(value: V) -> Self {
        Self { value }
    }
}

impl<V> Container for One<V> {
    type Key = usize;
    type Value = V;
}

impl<V> WithOne<V> for One<V> {
    #[inline(always)]
    fn with_one(value: V) -> Self {
        Self { value }
    }
}

impl<V> Assign for One<V> {
    #[inline(always)]
    fn assign(&mut self, value: Self) {
        *self = value;
    }
}

impl<V> Get<usize> for One<V> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        if *index == 0 { Some(&self.value) } else { None }
    }
}

impl<V> Set<usize> for One<V> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) -> Option<V> {
        assert_eq!(index, 0);
        Some(core::mem::replace(&mut self.value, value))
    }
}

impl<V> Modify<usize> for One<V> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, mut f: F)
    where
        F: FnMut(&mut V),
    {
        assert_eq!(*index, 0);
        f(&mut self.value)
    }
}

// No implementation of `Remove` because there is always exactly one element.
// Removing an element would violate this invariant property.

impl<V> Put<V> for One<V> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        Some(core::mem::replace(&mut self.value, value))
    }
}

// No implementation of `Clear` for the same reason as `Remove`.

impl<V> Len for One<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        1
    }
}

impl<'a, V: 'a> Values<'a> for One<V> {
    type Values = core::iter::Once<&'a V>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        core::iter::once(&self.value)
    }
}

impl<V> IntoValues for One<V> {
    type IntoValues = core::iter::Once<V>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        core::iter::once(self.value)
    }
}

impl<'a, V: 'a> Iter<'a, usize> for One<V> {
    type Iter = core::iter::Once<(usize, &'a V)>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        core::iter::once((0, &self.value))
    }
}

impl<V> IntoIter<usize> for One<V> {
    type IntoIter = core::iter::Enumerate<core::iter::Once<V>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        core::iter::once(self.value).enumerate()
    }
}
