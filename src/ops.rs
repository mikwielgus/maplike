// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Individual container operation traits.

use core::borrow::Borrow;

use crate::containers::Container;

/// Construct a container with exactly one element.
pub trait WithOne<E>: Container {
    /// Construct a container containing only the given element.
    fn with_one(element: E) -> Self;
}

/// Replace self with a new value.
///
/// # Examples
///
/// ```
/// use maplike::ops::Assign;
/// use std::collections::HashMap;
///
/// // `.assign()` replaces the whole value with a new one.
///
/// // Works for scalars.
/// let mut count = 1;
/// count.assign(42);
/// assert_eq!(count, 42);
///
/// // Works for tuples.
/// let mut point = (1, 2);
/// point.assign((10, 20));
/// assert_eq!(point, (10, 20));
///
/// // Works for `Vec`.
/// let mut vec = vec![1, 2, 3];
/// vec.assign(vec![4, 5, 6]);
/// assert_eq!(vec, [4, 5, 6]);
///
/// // Works for `HashMap`.
/// let mut map = HashMap::from([(1, 2.0)]);
/// map.assign(HashMap::from([(3, 4.0), (5, 6.0)]));
/// assert_eq!(map.get(&3), Some(&4.0));
/// assert_eq!(map.get(&1), None);
/// ```
pub trait Assign<V = Self>: Container {
    /// Replace self with a new value.
    fn assign(&mut self, value: V);
}

/// Returns a reference to the value corresponding to the key.
///
/// # Examples
///
/// ```
/// use maplike::ops::Get;
/// use std::collections::{BTreeMap, HashMap};
///
/// // Generic over any collection implementing `Get`.
/// fn get_second_element<C: Get<usize>>(collection: &C) -> Option<&C::Value> {
///     collection.get(&1)
/// }
///
/// // `get_second_element()` works for `Vec`.
/// assert_eq!(get_second_element(&vec![10, 20, 30]), Some(&20));
///
/// // `get_second_element()` works for `[T; N]`.
/// assert_eq!(get_second_element(&[10, 20, 30]), Some(&20));
///
/// // `get_second_element()` works for `HashMap`.
/// assert_eq!(
///     get_second_element(&HashMap::from([(0, 10), (1, 20)])),
///     Some(&20),
/// );
///
/// // `get_second_element()` works for `BTreeMap`.
/// assert_eq!(
///     get_second_element(&BTreeMap::from([(0, 10), (1, 20)])),
///     Some(&20),
/// );
/// ```
pub trait Get<K, Q: ?Sized = K>: Container
where
    K: Borrow<Q>,
{
    /// Returns a reference to the value corresponding to the key.
    fn get(&self, key: &Q) -> Option<&Self::Value>;
}

/// Returns a reference to the right value corresponding to the given left value
/// in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`GetByRight::get_by_right()`], and should
/// behave identically to [`Get`].
///
/// # Examples
///
/// ```
/// use maplike::ops::GetByLeft;
///
/// #[cfg(feature = "bidimap")]
/// {
///     use bidimap::{BiBTreeMap, BiHashMap};
///
///     // Works for `BiBTreeMap`.
///     let mut btreemap = BiBTreeMap::<String, i32>::new();
///     btreemap.insert("east".to_string(), 1);
///     btreemap.insert("west".to_string(), 2);
///     assert_eq!(btreemap.get_by_left("east"), Some(&1));
///
///     // Works for `BiHashMap`.
///     let mut hashmap = BiHashMap::<String, i32>::new();
///     hashmap.insert("east".to_string(), 1);
///     hashmap.insert("west".to_string(), 2);
///     assert_eq!(hashmap.get_by_left("west"), Some(&2));
/// }
/// ```
pub trait GetByLeft<K, Q: ?Sized = K>: Container
where
    K: Borrow<Q>,
{
    /// Returns a reference to the right value corresponding to the given left value.
    ///
    /// Should be only implemented only for bidirectional maps (not for
    /// unidirectional maps) along with [`GetByRight::get_by_right()`], and
    /// should behave identically to [`Get`].
    fn get_by_left(&self, key: &Q) -> Option<&Self::Value>;
}

/// Returns a reference to the left value corresponding to the given right value
/// in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`GetByLeft::get_by_left()`].
///
/// Note that key and value are unusually inverted here: `Self::Value` is
/// actually the key, while `K` is the value.
///
/// # Examples
///
/// ```
/// use maplike::ops::GetByRight;
///
/// #[cfg(feature = "bidimap")]
/// {
///     use bidimap::{BiBTreeMap, BiHashMap};
///
///     // Works for `BiBTreeMap`.
///     let mut btreemap = BiBTreeMap::<String, i32>::new();
///     btreemap.insert("east".to_string(), 1);
///     btreemap.insert("west".to_string(), 2);
///     assert_eq!(btreemap.get_by_right(&2), Some(&"west".to_string()));
///
///     // Works for `BiHashMap`.
///     let mut hashmap = BiHashMap::<String, i32>::new();
///     hashmap.insert("east".to_string(), 1);
///     hashmap.insert("west".to_string(), 2);
///     assert_eq!(hashmap.get_by_right(&1), Some(&"east".to_string()));
/// }
/// ```
pub trait GetByRight<K, Q: ?Sized = <Self as Container>::Value>: Container
where
    Self::Value: Borrow<Q>,
{
    /// Returns a reference to the right value corresponding to the given left value.
    ///
    /// Should be only implemented only for bidirectional maps (not for
    /// unidirectional maps) along with [`GetByLeft::get_by_left()`].
    ///
    /// Note that key and value are unusually inverted here: `Self::Value` is
    /// actually the key, while `K` is the value.
    fn get_by_right(&self, key: &Q) -> Option<&K>;
}

/// Set the value of an already existing element under a key.
///
/// Unlike [`insert`](Insert::insert), the key must already exist in the
/// container.
///
/// # Examples
///
/// ```
/// use maplike::ops::{Get, Set};
/// use std::collections::{BTreeMap, HashMap};
///
/// // Works for `Vec`.
/// let mut vec = vec![1.0, 2.0, 3.0];
/// vec.set(1, 20.0);
/// assert_eq!(vec.get(&1), Some(&20.0));
///
/// // Works for `[T; N]`.
/// let mut arr = [1.0, 2.0, 3.0];
/// arr.set(1, 20.0);
/// assert_eq!(arr.get(&1), Some(&20.0));
///
/// // Works for `HashMap`.
/// let mut hash_map = HashMap::from([(1, 2.0), (2, 3.0)]);
/// hash_map.set(1, 20.0);
/// assert_eq!(hash_map.get(&1), Some(&20.0));
///
/// // Works for `BTreeMap`.
/// let mut btree_map = BTreeMap::from([(1, 2.0), (2, 3.0)]);
/// btree_map.set(1, 20.0);
/// assert_eq!(btree_map.get(&1), Some(&20.0));
/// ```
pub trait Set<K>: Container {
    /// Return type of [`set`](Set::set).
    type Output;

    /// Set the value of an already existing element under a key.
    ///
    /// Unlike [`insert`](Insert::insert), the key must already exist in the
    /// container.
    fn set(&mut self, key: K, value: Self::Value) -> Self::Output;
}

/// Modify the value under key with a closure.
///
/// This is useful if something always has to be done before or after the
/// modification to maintain an invariant.
///
/// # Examples
///
/// ```
/// use maplike::ops::{Get, Modify};
/// use std::collections::{BTreeMap, HashMap};
///
/// // Works for `Vec`.
/// let mut vec = vec![1.0, 2.0, 3.0];
/// vec.modify(&1, |value| *value *= 10.0);
/// assert_eq!(vec.get(&1), Some(&20.0));
///
/// // Works for `[T; N]`.
/// let mut arr = [1.0, 2.0, 3.0];
/// arr.modify(&1, |value| *value *= 10.0);
/// assert_eq!(arr.get(&1), Some(&20.0));
///
/// // Works for `HashMap`.
/// let mut hash_map = HashMap::from([(1, 2.0), (2, 3.0)]);
/// hash_map.modify(&1, |value| *value *= 10.0);
/// assert_eq!(hash_map.get(&1), Some(&20.0));
///
/// // Works for `BTreeMap`.
/// let mut btree_map = BTreeMap::from([(1, 2.0), (2, 3.0)]);
/// btree_map.modify(&1, |value| *value *= 10.0);
/// assert_eq!(btree_map.get(&1), Some(&20.0));
/// ```
pub trait Modify<K, Q: ?Sized = K>: Container
where
    K: Borrow<Q>,
{
    /// Modify the value under key with a closure.
    ///
    /// This is useful if something always has to be done before or after the
    /// modification to maintain an invariant.
    fn modify<F>(&mut self, key: &Q, f: F)
    where
        F: FnMut(&mut Self::Value);
}

/// Insert a new key-value pair into the container at an arbitrary key.
///
/// The key can but does not have to already exist in the container.
///
/// # Examples
///
/// ```
/// use maplike::ops::{Get, Insert};
/// use std::collections::{BTreeMap, HashMap};
///
/// // Works for `HashMap`.
/// let mut hashmap = HashMap::new();
/// hashmap.insert(1, 2.0);
/// assert_eq!(hashmap.get(&1), Some(&2.0));
///
/// // Works for `BTreeMap`.
/// let mut btreemap = BTreeMap::new();
/// btreemap.insert(1, 2.0);
/// assert_eq!(btreemap.get(&1), Some(&2.0));
/// ```
pub trait Insert<K>: Container {
    /// Return type of [`insert`](Insert::insert).
    type Output;

    /// Insert a new key-value pair into the container at an arbitrary key.
    ///
    /// The key can but does not have to already exist in the container.
    fn insert(&mut self, key: K, value: Self::Value) -> Self::Output;
}

/// Remove an element under a key from the collection, returning the value
/// at the key if the key was previously in the map. Other keys are not
/// invalidated.
///
/// [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html) obviously does
/// not implement this trait because its element removal methods,
/// [`Vec::remove()`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.remove)
/// and
/// [`Vec::swap_remove()`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.swap_remove),
/// invalidate existing indices.
///
/// If you need this trait on a contiguous data type with constant-time
/// insertion, lookup, and removal, try
/// [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/type.StableVec.html)
/// or [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html).
///
/// # Examples
///
/// ```
/// use maplike::ops::{Get, Remove};
/// use std::collections::{BTreeMap, HashMap};
///
/// // Works for `HashMap`.
/// let mut hashmap = HashMap::from([(1, 2.0), (2, 3.0)]);
/// assert_eq!(hashmap.remove(&1), Some(2.0));
/// assert_eq!(hashmap.get(&1), None);
///
/// // Works for `BTreeMap`.
/// let mut btreemap = BTreeMap::from([(1, 2.0), (2, 3.0)]);
/// assert_eq!(btreemap.remove(&1), Some(2.0));
/// assert_eq!(btreemap.get(&1), None);
/// ```
pub trait Remove<K, Q: ?Sized = K>: Container
where
    K: Borrow<Q>,
{
    /// Return type of [`remove`](Remove::remove).
    type Output;

    /// Remove an element under a key from the collection, returning the value
    /// at the key if the key was previously in the map. Other keys are not
    /// invalidated.
    ///
    /// [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html) obviously
    /// does not implement this trait because its element removal methods,
    /// [`Vec::remove()`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.remove)
    /// and
    /// [`Vec::swap_remove()`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.swap_remove),
    /// invalidate existing indices.
    ///
    /// If you need this trait on a contiguous data type with constant-time
    /// insertion, lookup, and removal, try
    /// [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/type.StableVec.html)
    /// or [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html).
    fn remove(&mut self, key: &Q) -> Self::Output;
}

/// Remove the left and right values from pair corresponding to the given left
/// value in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`RemoveByRight::remove_by_right()`], and should
/// behave identically to [`Remove`].
///
/// # Examples
///
/// ```
/// use maplike::ops::{GetByLeft, RemoveByLeft};
///
/// #[cfg(feature = "bidimap")]
/// {
///     use bidimap::{BiBTreeMap, BiHashMap};
///
///     // Works for `BiBTreeMap`.
///     let mut bibtreemap = BiBTreeMap::<String, i32>::new();
///     bibtreemap.insert("east".to_string(), 1);
///     bibtreemap.insert("west".to_string(), 2);
///     assert_eq!(RemoveByLeft::remove_by_left(&mut bibtreemap, "east"), Some(1));
///     assert_eq!(bibtreemap.get_by_left("east"), None);
///
///     // Works for `BiHashMap`.
///     let mut bihashmap = BiHashMap::<String, i32>::new();
///     bihashmap.insert("east".to_string(), 1);
///     bihashmap.insert("west".to_string(), 2);
///     assert_eq!(RemoveByLeft::remove_by_left(&mut bihashmap, "west"), Some(2));
///     assert_eq!(bihashmap.get_by_left("west"), None);
/// }
/// ```
pub trait RemoveByLeft<K, Q: ?Sized = K>: Container
where
    K: Borrow<Q>,
{
    /// Remove the left and right values from pair corresponding to the given
    /// left value in a bidirectional map.
    ///
    /// Should be only implemented only for bidirectional maps (not for
    /// unidirectional maps) along with [`RemoveByRight::remove_by_right()`],
    /// and should behave identically to [`Remove`].
    fn remove_by_left(&mut self, key: &Q) -> Option<Self::Value>;
}

/// Remove the left and right values from pair corresponding to the given right
/// value in a bidirectional map.
///
/// Should be only implemented only for bidirectional maps (not for
/// unidirectional maps) along with [`RemoveByLeft::remove_by_left()`].
///
/// Note that key and value are unusually inverted here: `Self::Value` is
/// actually the key, while `K` is the value.
///
/// # Examples
///
/// ```
/// use maplike::ops::{GetByRight, RemoveByRight};
///
/// #[cfg(feature = "bidimap")]
/// {
///     use bidimap::{BiBTreeMap, BiHashMap};
///
///     // Works for `BiBTreeMap`.
///     let mut bibtreemap = BiBTreeMap::<String, i32>::new();
///     bibtreemap.insert("east".to_string(), 1);
///     bibtreemap.insert("west".to_string(), 2);
///     assert_eq!(RemoveByRight::remove_by_right(&mut bibtreemap, &2), Some("west".to_string()));
///     assert_eq!(bibtreemap.get_by_right(&2), None);
///
///     // Works for `BiHashMap`.
///     let mut bihashmap = BiHashMap::<String, i32>::new();
///     bihashmap.insert("east".to_string(), 1);
///     bihashmap.insert("west".to_string(), 2);
///     assert_eq!(RemoveByRight::remove_by_right(&mut bihashmap, &1), Some("east".to_string()));
///     assert_eq!(bihashmap.get_by_right(&1), None);
/// }
/// ```
pub trait RemoveByRight<K, Q: ?Sized = <Self as Container>::Value>: Container
where
    Self::Value: Borrow<Q>,
{
    /// Remove the left and right values from pair corresponding to the given
    /// right value in a bidirectional map.
    ///
    /// Should be only implemented only for bidirectional maps (not for
    /// unidirectional maps) along with [`RemoveByLeft::remove_by_left()`].
    ///
    /// Note that key and value are unusually inverted here: `Self::Value` is
    /// actually the key, while `K` is the value.
    fn remove_by_right(&mut self, key: &Q) -> Option<K>;
}

/// Insert a value into the collection without specifying a key, returning
/// the key that was automatically generated.
///
/// # Examples
///
/// ```
/// use maplike::ops::{Get, Push};
/// use std::collections::VecDeque;
///
/// // Works for `Vec`.
/// let mut vec = Vec::new();
/// assert_eq!(Push::push(&mut vec, 1.0), 0);
/// assert_eq!(Push::push(&mut vec, 2.0), 1);
/// assert_eq!(vec.get(&1), Some(&2.0));
///
/// // Works for `VecDeque`.
/// let mut deque = VecDeque::new();
/// assert_eq!(deque.push(1.0), 0);
/// assert_eq!(deque.push(2.0), 1);
/// assert_eq!(Get::get(&deque, &1), Some(&2.0));
/// ```
pub trait Push<K>: Container {
    /// Insert a value into the collection without specifying a key, returning
    /// the key that was automatically generated.
    fn push(&mut self, value: Self::Value) -> K;
}

/// Remove the last element of the collection, returning it.
///
/// If `Push` is also implemented, calling `Pop` should revert the previous
/// pushes in their reversed order.
///
/// # Examples
///
/// ```
/// use maplike::ops::{Pop, Push};
/// use std::collections::VecDeque;
///
/// // Works for `Vec`.
/// let mut vec = Vec::new();
/// Push::push(&mut vec, 1.0);
/// Push::push(&mut vec, 2.0);
/// assert_eq!(vec.pop(), Some(2.0));
/// assert_eq!(vec.pop(), Some(1.0));
///
/// // Works for `VecDeque`.
/// let mut deque = VecDeque::new();
/// deque.push(1.0);
/// deque.push(2.0);
/// assert_eq!(deque.pop(), Some(2.0));
/// assert_eq!(deque.pop(), Some(1.0));
/// ```
pub trait Pop: Container {
    /// Remove the last element of the collection, returning it.
    ///
    /// If `Push` is also implemented, calling `Pop` should revert the previous
    /// pushes in their reversed order.
    fn pop(&mut self) -> Option<Self::Value>;
}

/// Put a new value in the container.
///
/// This is basically [`push`](Push::push), but unlike it:
///
/// - it also works for sets,
/// - it does not matter what is the key,
/// - in some containers, `.put()` may evict elements.
///
/// If the insertion has happened to evict (aka. override or displace) an existing element,
/// this element is returned.
///
/// # Examples
///
/// ```
/// use maplike::ops::Put;
/// use std::collections::HashSet;
///
/// // Works for `Vec`.
/// let mut vec = Vec::new();
/// assert_eq!(vec.put(1.0), None);
/// assert_eq!(vec.put(2.0), None);
/// assert_eq!(vec, [1.0, 2.0]);
///
/// // Works for `HashSet`.
/// let mut set = HashSet::new();
/// assert_eq!(set.put(1), None);
/// assert_eq!(set.put(2), None);
/// assert_eq!(set.len(), 2);
///
/// // Works for scalars.
/// let mut count = 1;
/// assert_eq!(count.put(42), Some(1));
/// assert_eq!(count, 42);
/// ```
pub trait Put<E>: Container {
    /// Put a new value in the container.
    ///
    /// This is basically [`push`](Push::push), but unlike it:
    ///
    /// - it also works for sets,
    /// - it does not matter what is the key,
    /// - in some containers, `.put()` may evict elements.
    ///
    /// If the insertion has happened to evict (aka. override or displace) an existing element,
    /// this element is returned.
    fn put(&mut self, element: E) -> Option<E>;
}

/// Remove all elements from the collection.
///
/// # Examples
///
/// ```
/// use maplike::ops::{Clear, Get};
/// use std::collections::{BTreeMap, HashMap};
///
/// // Works for `Vec`.
/// let mut vec = vec![1.0, 2.0, 3.0];
/// vec.clear();
/// assert_eq!(vec.len(), 0);
///
/// // Works for `HashMap`.
/// let mut hashmap = HashMap::from([(1, 2.0), (2, 3.0)]);
/// hashmap.clear();
/// assert_eq!(hashmap.get(&1), None);
///
/// // Works for `BTreeMap`.
/// let mut btreemap = BTreeMap::from([(1, 2.0), (2, 3.0)]);
/// btreemap.clear();
/// assert_eq!(btreemap.get(&1), None);
/// ```
pub trait Clear: Container {
    /// Remove all elements from the collection.
    fn clear(&mut self);
}

/// Returns the number of elements in the collection.
///
/// # Examples
///
/// ```
/// use maplike::ops::Len;
/// use std::collections::{HashMap, VecDeque};
///
/// // Works for `Vec`.
/// let vec = vec![1.0, 2.0, 3.0];
/// assert_eq!(vec.len(), 3);
///
/// // Works for `[T; N]`.
/// let arr = [1.0, 2.0, 3.0];
/// assert_eq!(arr.len(), 3);
///
/// // Works for `VecDeque`.
/// let deque = VecDeque::from([1.0, 2.0, 3.0]);
/// assert_eq!(deque.len(), 3);
///
/// // Works for `HashMap`.
/// let hashmap = HashMap::from([(1, 2.0), (2, 3.0)]);
/// assert_eq!(hashmap.len(), 2);
/// ```
pub trait Len: Container {
    /// Returns the number of elements in the collection.
    fn len(&self) -> usize;
}

/// Resize the collection to the given length.
///
/// If `new_len` is greater than the current length, the collection is extended
/// by cloning `value` until its length equals `new_len`.
///
/// If `new_len` is less than the current length, the collection is truncated.
///
/// # Examples
///
/// ```
/// use maplike::ops::{Get, Len, Resize};
///
/// // Works for `Vec`.
/// let mut vec = vec![1.0, 2.0];
/// vec.resize(4, 0.0);
/// assert_eq!(vec.len(), 4);
/// assert_eq!(vec.get(&3), Some(&0.0));
///
/// vec.resize(1, 0.0);
/// assert_eq!(vec.len(), 1);
/// assert_eq!(vec.get(&0), Some(&1.0));
/// ```
pub trait Resize: Container {
    /// Resize the collection to the given length.
    ///
    /// If `new_len` is greater than the current length, the collection is extended
    /// by cloning `value` until its length equals `new_len`.
    ///
    /// If `new_len` is less than the current length, the collection is truncated.
    fn resize(&mut self, new_len: usize, value: Self::Value)
    where
        Self::Value: Clone;
}
