// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Abstract container traits that bundle multiple operations together.

use core::ops::Index;

use crate::ops::{Clear, Get, Insert, Len, Pop, Push, Remove, Set};

/// Base trait for keyed collections, without any operations defined yet.
///
/// Just a key-value map without any methods yet. We however use the name
/// `Container` instead of `Map` to distinguish maps from vectors and stable
/// vectors, which also are keyed collections but with slightly different sets
/// of operations.
pub trait Container {
    /// Type of the keys in the keyed collection.
    type Key;
    /// Type of the values in the keyed collection.
    type Value;
}

/// A single assignable value.
///
/// # Examples
///
/// ```
/// use maplike::ops::Assign;
///
/// // `.assign()` replaces the whole value for any `Scalarlike` type.
///
/// // Works for `i32`.
/// let mut count = 1;
/// count.assign(42);
/// assert_eq!(count, 42);
///
/// // Works for `f64`.
/// let mut ratio = 1.0;
/// ratio.assign(3.5);
/// assert_eq!(ratio, 3.5);
///
/// // Works for tuples.
/// let mut point = (1, 2);
/// point.assign((10, 20));
/// assert_eq!(point, (10, 20));
/// ```
pub use crate::ops::Assign as Scalarlike;

/// A keyed collection with get, set, insert, remove, clear operations.
///
/// # Examples
///
/// ```
/// use maplike::containers::Maplike;
/// use maplike::ops::{Get, Insert, Set};
/// use std::collections::{BTreeMap, HashMap};
///
/// // Generic over any `Maplike` collection.
/// fn scale_entry<C: Maplike<usize, Value = f64>>(map: &mut C, key: usize, factor: f64) {
///     if let Some(&value) = map.get(&key) {
///         // Scale an existing entry.
///         map.set(key, value * factor);
///     } else {
///         // Insert a new entry when the key is absent.
///         map.insert(key, factor);
///     }
/// }
///
/// // `scale_entry()` works on `HashMap`.
/// let mut hash_map = HashMap::from([(1, 2.0), (2, 3.0)]);
/// scale_entry(&mut hash_map, 1, 10.0);
/// assert_eq!(hash_map.get(&1), Some(&20.0));
/// scale_entry(&mut hash_map, 3, 5.0);
/// assert_eq!(hash_map.get(&3), Some(&5.0));
///
/// // `scale_entry()` works on `BTreeMap`.
/// let mut btree_map = BTreeMap::from([(1, 2.0), (2, 3.0)]);
/// scale_entry(&mut btree_map, 1, 10.0);
/// assert_eq!(btree_map.get(&1), Some(&20.0));
/// scale_entry(&mut btree_map, 3, 5.0);
/// assert_eq!(btree_map.get(&3), Some(&5.0));
/// ```
pub trait Maplike<K>: Get<K> + Set<K> + Insert<K> + Remove<K> + Clear
where
    for<'a> Self: Index<&'a K>,
{
}
impl<K, T: Get<K> + Set<K> + Insert<K> + Remove<K> + Clear> Maplike<K> for T where
    for<'a> Self: Index<&'a K>
{
}

/// A map-like keyed collection whose value is the unit type, thus behaving like
/// a set.
pub trait Setlike<K>: Maplike<K, Value = ()> {}
impl<K, T: Maplike<K, Value = ()>> Setlike<K> for T {}

/// An array-like keyed collection with get, set, len, index operations.
///
/// # Examples
///
/// ```
/// use maplike::containers::Arraylike;
/// use maplike::ops::{Get, Len, Set};
///
/// // Generic over any `Arraylike` collection.
/// fn scale_all<C: Arraylike<usize, Key = usize, Value = f64>>(collection: &mut C, factor: f64) {
///     for i in 0..collection.len() {
///         if let Some(&value) = collection.get(&i) {
///             // Multiply all collection elements by a constant factor.
///             collection.set(i, value * factor);
///         }
///     }
/// }
///
/// // `scale_all()` works on `[T; N]`.
/// let mut arr = [1.0, 2.0, 3.0];
/// scale_all(&mut arr, 10.0);
/// assert_eq!(arr, [10.0, 20.0, 30.0]);
///
/// // `scale_all()` works on `Vec`.
/// let mut vec = vec![4.0, 5.0, 6.0];
/// scale_all(&mut vec, 10.0);
/// assert_eq!(vec, [40.0, 50.0, 60.0]);
///
/// use std::collections::VecDeque;
///
/// // `scale_all()` works on `VecDeque`.
/// let mut deque = VecDeque::from([7.0, 8.0, 9.0]);
/// scale_all(&mut deque, 10.0);
/// assert_eq!(deque.into_iter().collect::<Vec<_>>(), vec![70.0, 80.0, 90.0]);
/// ```
pub trait Arraylike<K>: Index<K> + Get<K> + Set<K> + Len {}
impl<K, T: Index<K> + Get<K> + Set<K> + Len> Arraylike<K> for T {}

/// An array-like keyed collection with additional push, pop, clear operations.
///
/// # Examples
///
/// ```
/// use maplike::containers::Veclike;
/// use maplike::ops::{Get, Len, Set};
///
/// // Generic over any `Veclike` collection.
/// fn scale_all<C: Veclike<usize, Key = usize, Value = f64>>(collection: &mut C, factor: f64) {
///     for i in 0..collection.len() {
///         if let Some(&value) = collection.get(&i) {
///             // Multiply all collection elements by a constant factor.
///             collection.set(i, value * factor);
///         }
///     }
/// }
///
/// // `scale_all()` works on `Vec`.
/// let mut vec = vec![4.0, 5.0, 6.0];
/// scale_all(&mut vec, 10.0);
/// assert_eq!(vec, [40.0, 50.0, 60.0]);
///
/// use std::collections::VecDeque;
///
/// // `scale_all()` works on `VecDeque`.
/// let mut deque = VecDeque::from([7.0, 8.0, 9.0]);
/// scale_all(&mut deque, 10.0);
/// assert_eq!(deque.into_iter().collect::<Vec<_>>(), vec![70.0, 80.0, 90.0]);
/// ```
pub trait Veclike<K>: Index<K> + Get<K> + Set<K> + Push<K> + Pop + Clear + Len {}
impl<K, T: Arraylike<K> + Push<K> + Pop + Clear> Veclike<K> for T {}
