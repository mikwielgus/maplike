// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Iteration traits for map-like containers.

use crate::abc::Keyed;

/// Borrow the collection and yield values.
pub trait Values<'a>: Keyed
where
    Self: 'a,
{
    /// Iterator that borrows values from the collection.
    type Values: Iterator<Item = &'a Self::Value>;

    /// Borrow the collection and yield values.
    fn values(&'a self) -> Self::Values;
}

/// Consume the collection and yield owned values.
pub trait IntoValues: Keyed {
    /// Iterator that consumes the collection and yields owned values.
    type IntoValues: Iterator<Item = Self::Value>;

    /// Consume the collection and yield owned values.
    fn into_values(self) -> Self::IntoValues;
}

/// Borrow the collection and yield key-value pairs.
pub trait Iter<'a, K>: Keyed
where
    Self: 'a,
{
    /// Iterator that borrows from the collection.
    type Iter: Iterator<Item = (K, &'a Self::Value)>;

    /// Borrow the collection and yield key-value pairs.
    fn iter(&'a self) -> Self::Iter;
}

/// Consume the collection and yield owned key-value pairs.
pub trait IntoIter<K>: Keyed {
    /// Iterator that consumes the collection.
    type IntoIter: Iterator<Item = (K, Self::Value)>;

    /// Consume the collection and yield owned key-value pairs.
    fn into_iter(self) -> Self::IntoIter;
}

/// Adapter that turns an iterator of key-value pairs (`(K, V)`) into an
/// iterator of values.
///
/// Useful for types that do not have `.values()`, `.into_values()`, or
/// value-yielding `.iter()`, `.into_iter()` but only have iterators over
/// key-value pairs.
pub struct ValuesFromKeyValuePairs<I>(pub I);

impl<I, K, V> Iterator for ValuesFromKeyValuePairs<I>
where
    I: Iterator<Item = (K, V)>,
{
    type Item = V;

    #[inline(always)]
    fn next(&mut self) -> Option<V> {
        self.0.next().map(|(_, value)| value)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}
