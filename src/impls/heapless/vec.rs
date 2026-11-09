// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use heapless::Vec;

use crate::abc::{Container, Keyed};
use crate::ctors::WithOne;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{CheckedPush, Clear, ContainsKey, Get, Len, Modify, Pop, Set, ShiftRemove};

impl<T, const N: usize> Container for Vec<T, N> {
    type Value = T;
}

impl<T, const N: usize> Keyed for Vec<T, N> {
    type Key = usize;
}

impl<T, const N: usize> WithOne<T> for Vec<T, N> {
    #[inline(always)]
    fn with_one(element: T) -> Self {
        let mut heapless_vec = Vec::new();

        // We usually avoid panicking due to exceeding bounds, preferring
        // checked interfaces. But here exceeding the capacity only happens in a
        // pathological case where `heapless::Vec`'s compile-time is zero.
        if Vec::push(&mut heapless_vec, element).is_err() {
            // Panic, not `.expect()`, because the latter would require `T: Debug`.
            panic!("compile-time capacity was set to 0, which is invalid")
        }

        heapless_vec
    }
}

impl<T, const N: usize> ContainsKey<usize> for Vec<T, N> {
    #[inline(always)]
    fn contains_key(&self, index: &usize) -> bool {
        *index < self.len()
    }
}

impl<T, const N: usize> Get<usize> for Vec<T, N> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&T> {
        self.as_slice().get(*index)
    }
}

impl<T, const N: usize> Set<usize> for Vec<T, N> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: T) {
        self[index] = value;
    }
}

impl<T, const N: usize> Modify<usize> for Vec<T, N> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self[*index]);
    }
}

impl<T, const N: usize> CheckedPush<usize> for Vec<T, N> {
    #[inline(always)]
    fn checked_push(&mut self, value: T) -> Option<usize> {
        let index = self.len();
        if Vec::push(self, value).is_ok() {
            Some(index)
        } else {
            None
        }
    }
}

impl<T, const N: usize> Pop for Vec<T, N> {
    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        Vec::pop(self)
    }
}

impl<T, const N: usize> ShiftRemove<usize> for Vec<T, N> {
    type Output = T;

    #[inline(always)]
    fn shift_remove(&mut self, key: &usize) -> T {
        Vec::remove(self, *key)
    }
}

impl<T, const N: usize> Clear for Vec<T, N> {
    #[inline(always)]
    fn clear(&mut self) {
        Vec::clear(self);
    }
}

impl<T, const N: usize> Len for Vec<T, N> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<'a, T: 'a, const N: usize> Values<'a> for Vec<T, N> {
    type Values = core::slice::Iter<'a, T>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        self.as_slice().iter()
    }
}

impl<T, const N: usize> IntoValues for Vec<T, N> {
    type IntoValues = <Vec<T, N> as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, T: 'a, const N: usize> Iter<'a, usize> for Vec<T, N> {
    type Iter = core::iter::Enumerate<core::slice::Iter<'a, T>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.as_slice().iter().enumerate()
    }
}

impl<T, const N: usize> IntoIter<usize> for Vec<T, N> {
    type IntoIter = core::iter::Enumerate<<Vec<T, N> as IntoIterator>::IntoIter>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
