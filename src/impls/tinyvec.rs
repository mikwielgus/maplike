// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use tinyvec::{Array, ArrayVec, TinyVec};

use crate::abc::Keyed;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Len, Modify, Pop, Push, Put, Resize, Set, WithOne};

impl<A: Array> Keyed for ArrayVec<A> {
    type Key = usize;
    type Value = A::Item;
}

impl<A: Array> WithOne<A::Item> for ArrayVec<A> {
    #[inline(always)]
    fn with_one(element: A::Item) -> Self {
        let mut array_vec = ArrayVec::new();
        ArrayVec::push(&mut array_vec, element);

        array_vec
    }
}

impl<A: Array> Get<usize> for ArrayVec<A> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&A::Item> {
        self.as_slice().get(*index)
    }
}

impl<A: Array> Set<usize> for ArrayVec<A> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: A::Item) {
        self[index] = value;
    }
}

impl<A: Array> Modify<usize> for ArrayVec<A> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut A::Item),
    {
        f(&mut self[*index]);
    }
}

impl<A: Array> Push<usize> for ArrayVec<A> {
    #[inline(always)]
    fn push(&mut self, value: A::Item) -> usize {
        ArrayVec::push(self, value);

        self.len() - 1
    }
}

impl<A: Array> Pop for ArrayVec<A> {
    #[inline(always)]
    fn pop(&mut self) -> Option<A::Item> {
        ArrayVec::pop(self)
    }
}

impl<A: Array> Put<A::Item> for ArrayVec<A> {
    #[inline(always)]
    fn put(&mut self, value: A::Item) -> Option<A::Item> {
        ArrayVec::push(self, value);

        None
    }
}

impl<A: Array> Clear for ArrayVec<A> {
    #[inline(always)]
    fn clear(&mut self) {
        ArrayVec::clear(self);
    }
}

impl<A: Array> Len for ArrayVec<A> {
    #[inline(always)]
    fn len(&self) -> usize {
        ArrayVec::len(self)
    }
}

impl<A: Array> Resize for ArrayVec<A> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: A::Item)
    where
        A::Item: Clone,
    {
        ArrayVec::resize(self, new_len, value);
    }
}

impl<'a, A: Array + 'a> Values<'a> for ArrayVec<A>
where
    A::Item: 'a,
{
    type Values = core::slice::Iter<'a, A::Item>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        self.as_slice().iter()
    }
}

impl<A: Array> IntoValues for ArrayVec<A> {
    type IntoValues = <ArrayVec<A> as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, A: Array + 'a> Iter<'a, usize> for ArrayVec<A>
where
    A::Item: 'a,
{
    type Iter = core::iter::Enumerate<core::slice::Iter<'a, A::Item>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.as_slice().iter().enumerate()
    }
}

impl<A: Array> IntoIter<usize> for ArrayVec<A> {
    type IntoIter = core::iter::Enumerate<<ArrayVec<A> as IntoIterator>::IntoIter>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}

impl<A: Array> Keyed for TinyVec<A> {
    type Key = usize;
    type Value = A::Item;
}

impl<A: Array> WithOne<A::Item> for TinyVec<A> {
    #[inline(always)]
    fn with_one(element: A::Item) -> Self {
        let mut tiny_vec = TinyVec::new();
        TinyVec::push(&mut tiny_vec, element);

        tiny_vec
    }
}

impl<A: Array> Get<usize> for TinyVec<A> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&A::Item> {
        self.as_slice().get(*index)
    }
}

impl<A: Array> Set<usize> for TinyVec<A> {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: A::Item) {
        self[index] = value;
    }
}

impl<A: Array> Modify<usize> for TinyVec<A> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut A::Item),
    {
        f(&mut self[*index]);
    }
}

impl<A: Array> Push<usize> for TinyVec<A> {
    #[inline(always)]
    fn push(&mut self, value: A::Item) -> usize {
        TinyVec::push(self, value);

        self.len() - 1
    }
}

impl<A: Array> Pop for TinyVec<A> {
    #[inline(always)]
    fn pop(&mut self) -> Option<A::Item> {
        TinyVec::pop(self)
    }
}

impl<A: Array> Put<A::Item> for TinyVec<A> {
    #[inline(always)]
    fn put(&mut self, value: A::Item) -> Option<A::Item> {
        TinyVec::push(self, value);

        None
    }
}

impl<A: Array> Clear for TinyVec<A> {
    #[inline(always)]
    fn clear(&mut self) {
        TinyVec::clear(self);
    }
}

impl<A: Array> Len for TinyVec<A> {
    #[inline(always)]
    fn len(&self) -> usize {
        TinyVec::len(self)
    }
}

impl<A: Array> Resize for TinyVec<A> {
    #[inline(always)]
    fn resize(&mut self, new_len: usize, value: A::Item)
    where
        A::Item: Clone,
    {
        TinyVec::resize(self, new_len, value);
    }
}

impl<'a, A: Array + 'a> Values<'a> for TinyVec<A>
where
    A::Item: 'a,
{
    type Values = core::slice::Iter<'a, A::Item>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        self.as_slice().iter()
    }
}

impl<A: Array> IntoValues for TinyVec<A> {
    type IntoValues = <TinyVec<A> as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        IntoIterator::into_iter(self)
    }
}

impl<'a, A: Array + 'a> Iter<'a, usize> for TinyVec<A>
where
    A::Item: 'a,
{
    type Iter = core::iter::Enumerate<core::slice::Iter<'a, A::Item>>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        self.as_slice().iter().enumerate()
    }
}

impl<A: Array> IntoIter<usize> for TinyVec<A> {
    type IntoIter = core::iter::Enumerate<<TinyVec<A> as IntoIterator>::IntoIter>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self).enumerate()
    }
}
