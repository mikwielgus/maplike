// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use arrayvec::ArrayVec;

use crate::containers::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Len, Modify, Pop, Push, Put, Set, WithOne};

impl<T, const CAP: usize> Container for ArrayVec<T, CAP> {
    type Key = usize;
    type Value = T;
}

impl<T, const CAP: usize> WithOne<T> for ArrayVec<T, CAP> {
    #[inline(always)]
    fn with_one(element: T) -> Self {
        let mut array_vec = ArrayVec::new();
        ArrayVec::push(&mut array_vec, element);

        array_vec
    }
}

impl<T, const CAP: usize> Get<usize> for ArrayVec<T, CAP> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&T> {
        self.as_slice().get(*index)
    }
}

impl<T, const CAP: usize> Set<usize> for ArrayVec<T, CAP> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: T) {
        self[index] = value;
    }
}

impl<T, const CAP: usize> Modify<usize> for ArrayVec<T, CAP> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self[*index]);
    }
}

impl<T, const CAP: usize> Push<usize> for ArrayVec<T, CAP> {
    #[inline(always)]
    fn push(&mut self, value: T) -> usize {
        ArrayVec::push(self, value);

        self.len() - 1
    }
}

impl<T, const CAP: usize> Pop for ArrayVec<T, CAP> {
    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        ArrayVec::pop(self)
    }
}

impl<T, const CAP: usize> Put<T> for ArrayVec<T, CAP> {
    #[inline(always)]
    fn put(&mut self, value: T) -> Option<T> {
        ArrayVec::push(self, value);

        None
    }
}

impl<T, const CAP: usize> Clear for ArrayVec<T, CAP> {
    #[inline(always)]
    fn clear(&mut self) {
        ArrayVec::clear(self);
    }
}

impl<T, const CAP: usize> Len for ArrayVec<T, CAP> {
    #[inline(always)]
    fn len(&self) -> usize {
        ArrayVec::len(self)
    }
}

impl<'a, T: 'a, const CAP: usize> Values<'a> for ArrayVec<T, CAP> {
    type Values = core::slice::Iter<'a, T>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        self.as_slice().iter()
    }
}

impl<T, const CAP: usize> IntoValues for ArrayVec<T, CAP> {
    type IntoValues = <ArrayVec<T, CAP> as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, T: 'a, const CAP: usize> Iter<'a, usize> for ArrayVec<T, CAP> {
    type Iter = core::iter::Enumerate<core::slice::Iter<'a, T>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.as_slice().iter().enumerate()
    }
}

impl<T, const CAP: usize> IntoIter<usize> for ArrayVec<T, CAP> {
    type IntoIter = core::iter::Enumerate<<ArrayVec<T, CAP> as IntoIterator>::IntoIter>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
