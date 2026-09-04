// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::abc::Container;
use crate::ops::{Get, Len, Modify, Set};

macro_rules! impl_container_for_tuple {
    ($($idx:tt $typ:ident),+) => {
        impl<$($typ,)+> Container for ($($typ,)+) {
            type Key = usize;
            type Value = Self;
        }

        impl<$($typ,)+> Len for ($($typ,)+) {
            #[inline(always)]
            fn len(&self) -> usize {
                1
            }
        }
    };
}

// Tuples with up to 12 elements are supported.
impl_container_for_tuple!(0 T0);
impl_container_for_tuple!(0 T0, 1 T1);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10);
impl_container_for_tuple!(0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11);

impl<V, const N: usize> Container for [V; N] {
    type Key = usize;
    type Value = V;
}

impl<V, const N: usize> Get<usize> for [V; N] {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        Some(&self[*index])
    }
}

impl<V, const N: usize> Set<usize> for [V; N] {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V, const N: usize> Modify<usize> for [V; N] {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(&mut self[*index]);
    }
}

impl<V, const N: usize> Len for [V; N] {
    #[inline(always)]
    fn len(&self) -> usize {
        N
    }
}

impl<V> Container for [V] {
    type Key = usize;
    type Value = V;
}

impl<V> Get<usize> for [V] {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        <[V]>::get(self, *index)
    }
}

impl<V> Set<usize> for [V] {
    type Output = ();

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) {
        self[index] = value;
    }
}

impl<V> Modify<usize> for [V] {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(&mut self[*index]);
    }
}

impl<V> Len for [V] {
    #[inline(always)]
    fn len(&self) -> usize {
        <[V]>::len(self)
    }
}
