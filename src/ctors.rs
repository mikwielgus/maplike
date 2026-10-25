// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Container constructor traits.

/// Construct a container with at least the specified capacity.
pub trait WithCapacity {
    /// Construct a container with at least the specified capacity.
    fn with_capacity(capacity: usize) -> Self;
}

/// Construct a hashing container with the given hasher to hash keys.
pub trait WithHasher<S> {
    /// Construct a hashing container with the given hasher to hash keys.
    fn with_hasher(hasher: S) -> Self;
}

/// Construct a hashing container with at least the specified capacity and with
/// given hasher to hash keys.
pub trait WithCapacityAndHasher<S> {
    /// Construct a hashing container with at least the specified capacity and with
    /// given hasher to hash keys.
    fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self;
}

/// Construct a container with exactly one element.
pub trait WithOne<E> {
    /// Construct a container containing only the given element.
    fn with_one(element: E) -> Self;
}
