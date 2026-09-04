// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use stable_vec::StableVecFacade;

use crate::abc::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values, ValuesFromKeyValuePairs};
use crate::ops::{Clear, Get, Insert, Len, Modify, Push, Put, Remove, Set, WithOne};

impl<V, C: stable_vec::core::Core<V>> Container for StableVecFacade<V, C> {
    type Key = usize;
    type Value = V;
}

impl<V, C: stable_vec::core::Core<V>> WithOne<V> for StableVecFacade<V, C> {
    #[inline(always)]
    fn with_one(element: V) -> Self {
        let mut stable_vec = StableVecFacade::new();
        StableVecFacade::push(&mut stable_vec, element);

        stable_vec
    }
}

impl<V, C: stable_vec::core::Core<V>> Get<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn get(&self, index: &usize) -> Option<&V> {
        StableVecFacade::get(self, *index)
    }
}

impl<V, C: stable_vec::core::Core<V>> Set<usize> for StableVecFacade<V, C> {
    type Output = Option<V>;

    #[inline(always)]
    fn set(&mut self, index: usize, value: V) -> Option<V> {
        StableVecFacade::insert(self, index, value)
    }
}

impl<V, C: stable_vec::core::Core<V>> Modify<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn modify<F>(&mut self, index: &usize, f: F)
    where
        F: FnOnce(&mut V),
    {
        f(self.get_mut(*index).expect("no value under key"));
    }
}

impl<V, C: stable_vec::core::Core<V>> Insert<usize> for StableVecFacade<V, C> {
    type Output = Option<V>;

    #[inline(always)]
    fn insert(&mut self, index: usize, value: V) -> Option<V> {
        self.reserve_for(index);
        StableVecFacade::insert(self, index, value)
    }
}

impl<V, C: stable_vec::core::Core<V>> Remove<usize> for StableVecFacade<V, C> {
    type Output = Option<V>;

    #[inline(always)]
    fn remove(&mut self, index: &usize) -> Option<V> {
        self.get(*index)?;
        StableVecFacade::remove(self, *index)
    }
}

impl<V, C: stable_vec::core::Core<V>> Push<usize> for StableVecFacade<V, C> {
    #[inline(always)]
    fn push(&mut self, value: V) -> usize {
        StableVecFacade::push(self, value)
    }
}

impl<V, C: stable_vec::core::Core<V>> Put<V> for StableVecFacade<V, C> {
    #[inline(always)]
    fn put(&mut self, value: V) -> Option<V> {
        StableVecFacade::push(self, value);

        None
    }
}

impl<V, C: stable_vec::core::Core<V>> Clear for StableVecFacade<V, C> {
    #[inline(always)]
    fn clear(&mut self) {
        StableVecFacade::clear(self);
    }
}

impl<V, C: stable_vec::core::Core<V>> Len for StableVecFacade<V, C> {
    #[inline(always)]
    fn len(&self) -> usize {
        StableVecFacade::num_elements(self)
    }
}

impl<'a, V: 'a, C: stable_vec::core::Core<V> + 'a> Values<'a> for StableVecFacade<V, C> {
    type Values = stable_vec::iter::Values<'a, V, C>;

    #[inline(always)]
    fn values(&'a self) -> Self::Values {
        StableVecFacade::values(self)
    }
}

impl<V, C: stable_vec::core::Core<V>> IntoValues for StableVecFacade<V, C> {
    type IntoValues = ValuesFromKeyValuePairs<stable_vec::iter::IntoIter<V, C>>;

    #[inline(always)]
    fn into_values(self) -> Self::IntoValues {
        ValuesFromKeyValuePairs(IntoIterator::into_iter(self))
    }
}

impl<'a, V: 'a, C: stable_vec::core::Core<V> + 'a> Iter<'a, usize> for StableVecFacade<V, C> {
    type Iter = stable_vec::iter::Iter<'a, V, C>;

    #[inline(always)]
    fn iter(&'a self) -> Self::Iter {
        StableVecFacade::iter(self)
    }
}

impl<V, C: stable_vec::core::Core<V>> IntoIter<usize> for StableVecFacade<V, C> {
    type IntoIter = stable_vec::iter::IntoIter<V, C>;

    #[inline(always)]
    fn into_iter(self) -> stable_vec::iter::IntoIter<V, C> {
        IntoIterator::into_iter(self)
    }
}
