// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Entry API traits for map-like containers.

/// Gets the given key's corresponding entry in the container for in-place
/// manipulation.
///
/// # Examples
///
/// ```
/// use maplike::entry::{CombinedEntry, Entry};
/// use maplike::ops::Get;
/// use std::collections::{BTreeMap, HashMap};
///
/// // Works for `HashMap`.
/// let mut hashmap = HashMap::new();
/// *hashmap.entry(1).or_insert(2) = 3;
/// assert_eq!(hashmap.get(&1), Some(&3));
///
/// // Works for `BTreeMap`.
/// let mut btreemap = BTreeMap::new();
/// *btreemap.entry(1).or_insert(2) = 3;
/// assert_eq!(btreemap.get(&1), Some(&3));
/// ```
pub trait Entry<K> {
    /// Entry type returned by [`entry`](Entry::entry).
    type Entry<'a>
    where
        Self: 'a,
        K: 'a;

    /// Gets the given key's corresponding entry in the container for in-place
    /// manipulation.
    fn entry(&mut self, key: K) -> Self::Entry<'_>;
}

/// A view into a single entry in a map, which may either be vacant or occupied.
pub trait CombinedEntry<'a, K, V>: Sized {
    /// Occupied entry type returned by [`insert_entry`](CombinedEntry::insert_entry).
    type OccupiedEntry: OccupiedEntry<'a, K, V>;

    /// Returns a reference to this entry's key.
    fn key(&self) -> &K;

    /// Ensures a value is in the entry by inserting the default if empty, and
    /// returns a mutable reference to the value in the entry.
    fn or_insert(self, default: V) -> &'a mut V;

    /// Ensures a value is in the entry by inserting the result of the default
    /// function if empty, and returns a mutable reference to the value in the entry.
    fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V;

    /// Ensures a value is in the entry by inserting, if empty, the result of
    /// the default function.
    fn or_insert_with_key<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce(&K) -> V;

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V);

    /// Sets the value of the entry, and returns a `OccupiedEntry`.
    fn insert_entry(self, value: V) -> Self::OccupiedEntry;

    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    fn or_default(self) -> &'a mut V
    where
        V: Default;
}

/// A view into an occupied entry in a `BTreeMap`.
pub trait OccupiedEntry<'a, K, V> {
    /// Gets a reference to the key in the entry.
    fn key(&self) -> &K;

    /// Gets a reference to the value in the entry.
    fn get(&self) -> &V;

    /// Gets a mutable reference to the value in the entry.
    fn get_mut(&mut self) -> &mut V;

    /// Converts the entry into a mutable reference to its value.
    fn into_mut(self) -> &'a mut V;

    /// Sets the value of the entry with this view's key, and returns the
    /// entry's old value.
    fn insert(&mut self, value: V) -> V;

    /// Takes the value of the entry out of the map, and returns it.
    fn remove(self) -> V;

    /// Take ownership of the key and value from the map.
    fn remove_entry(self) -> (K, V);
}

/// A view into a vacant entry in a `BTreeMap`.
pub trait VacantEntry<'a, K, V> {
    /// Occupied entry type returned by [`insert_entry`](VacantEntry::insert_entry).
    type OccupiedEntry: OccupiedEntry<'a, K, V>;

    /// Gets a reference to the key that would be used when inserting a value
    /// through the VacantEntry.
    fn key(&self) -> &K;

    /// Take ownership of the key.
    fn into_key(self) -> K;

    /// Sets the value of the entry with this view's key, and returns a mutable
    /// reference to it.
    fn insert(self, value: V) -> &'a mut V;

    /// Sets the value of the entry with this view's key, and returns an
    /// `OccupiedEntry`.
    fn insert_entry(self, value: V) -> Self::OccupiedEntry;
}
