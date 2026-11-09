// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use heapless::Deque;

use crate::abc::{Container, Keyed};
use crate::ctors::WithOne;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{CheckedPush, Clear, ContainsKey, Get, Len, Modify, Pop, Set, SwapRemove};

impl<T, const N: usize> Container for Deque<T, N> {
    type Value = T;
}

impl<T, const N: usize> Keyed for Deque<T, N> {
    type Key = usize;
}

impl<T, const N: usize> WithOne<T> for Deque<T, N> {
    #[inline(always)]
    fn with_one(element: T) -> Self {
        let mut heapless_deque = Deque::new();

        // We usually avoid panicking due to exceeding bounds, preferring
        // checked interfaces. But here exceeding the capacity only happens in a
        // pathological case where `heapless::Vec`'s compile-time is zero.
        if Deque::push_back(&mut heapless_deque, element).is_err() {
            // Panic, not `.expect()`, because the latter would require `T: Debug`.
            panic!("compile-time capacity was set to 0, which is invalid")
        }

        heapless_deque
    }
}

impl<T, const N: usize> ContainsKey<usize> for Deque<T, N> {
    #[inline(always)]
    fn contains_key(&self, index: &usize) -> bool {
        *index < Deque::len(self)
    }
}

impl<T, const N: usize> Get<usize> for Deque<T, N> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&T> {
        Deque::get(self, *index)
    }
}

impl<T, const N: usize> Set<usize> for Deque<T, N> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: T) {
        *Deque::get_mut(self, index).expect("index out of bounds") = value;
    }
}

impl<T, const N: usize> Modify<usize> for Deque<T, N> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(Deque::get_mut(self, *index).expect("index out of bounds"));
    }
}

impl<T, const N: usize> CheckedPush<usize> for Deque<T, N> {
    #[inline(always)]
    fn checked_push(&mut self, value: T) -> Option<usize> {
        let index = Deque::len(self);

        if Deque::push_back(self, value).is_ok() {
            Some(index)
        } else {
            None
        }
    }
}

impl<T, const N: usize> Pop for Deque<T, N> {
    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        Deque::pop_back(self)
    }
}

impl<T, const N: usize> SwapRemove<usize> for Deque<T, N> {
    type Output = T;

    #[inline(always)]
    fn swap_remove(&mut self, key: &usize) -> T {
        Deque::swap_remove_back(self, *key).expect("index out of bounds")
    }
}

impl<T, const N: usize> Clear for Deque<T, N> {
    #[inline(always)]
    fn clear(&mut self) {
        Deque::clear(self);
    }
}

impl<T, const N: usize> Len for Deque<T, N> {
    #[inline(always)]
    fn len(&self) -> usize {
        Deque::len(self)
    }
}

impl<'a, T: 'a, const N: usize> Values<'a> for Deque<T, N> {
    type Values = heapless::deque::Iter<'a, T>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        Deque::iter(self)
    }
}

impl<T, const N: usize> IntoValues for Deque<T, N> {
    type IntoValues = heapless::deque::IntoIter<T, N>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, T: 'a, const N: usize> Iter<'a, usize> for Deque<T, N> {
    type Iter = core::iter::Enumerate<heapless::deque::Iter<'a, T>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        Deque::iter(self).enumerate()
    }
}

impl<T, const N: usize> IntoIter<usize> for Deque<T, N> {
    type IntoIter = core::iter::Enumerate<heapless::deque::IntoIter<T, N>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
