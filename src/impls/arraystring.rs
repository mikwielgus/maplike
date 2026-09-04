// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
use arrayvec::ArrayString;

use crate::abc::Keyed;
use crate::ops::{Clear, Len, Pop, Push, Put, WithOne};

impl<const CAP: usize> Keyed for ArrayString<CAP> {
    type Key = usize;
    type Value = char;
}

impl<const CAP: usize> WithOne<char> for ArrayString<CAP> {
    #[inline(always)]
    fn with_one(element: char) -> Self {
        let mut array_string = ArrayString::new();
        ArrayString::push(&mut array_string, element);

        array_string
    }
}

impl<const CAP: usize> Push<usize> for ArrayString<CAP> {
    #[inline(always)]
    fn push(&mut self, value: char) -> usize {
        let index = self.chars().count();
        ArrayString::push(self, value);

        index
    }
}

impl<const CAP: usize> Pop for ArrayString<CAP> {
    #[inline(always)]
    fn pop(&mut self) -> Option<char> {
        ArrayString::pop(self)
    }
}

impl<const CAP: usize> Put<char> for ArrayString<CAP> {
    #[inline(always)]
    fn put(&mut self, value: char) -> Option<char> {
        ArrayString::push(self, value);

        None
    }
}

impl<const CAP: usize> Clear for ArrayString<CAP> {
    #[inline(always)]
    fn clear(&mut self) {
        ArrayString::clear(self);
    }
}

impl<const CAP: usize> Len for ArrayString<CAP> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.chars().count()
    }
}
