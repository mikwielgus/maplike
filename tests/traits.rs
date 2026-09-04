// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// This file was generated using Claude Opus 4.8 Medium and Cursor Grok 4.5 with
// many manual and automated modifications.

#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::Debug;

use maplike::abc::Keyed;
use maplike::entry::{CombinedEntry, Entry, OccupiedEntry, VacantEntry};
use maplike::iter::IntoIter;
use maplike::ops::{
    Assign, Clear, Get, GetByLeft, GetByRight, Insert, Len, Modify, Pop, Push, Put, Remove,
    RemoveByLeft, RemoveByRight, Resize, Set, SwapRemove, WithOne,
};

trait FromUsize {
    fn from_usize(u: usize) -> Self;
}

impl FromUsize for () {
    fn from_usize(_: usize) {}
}

impl FromUsize for usize {
    fn from_usize(u: usize) -> usize {
        u
    }
}

impl FromUsize for i32 {
    fn from_usize(u: usize) -> i32 {
        u as i32
    }
}

impl FromUsize for String {
    fn from_usize(u: usize) -> String {
        u.to_string()
    }
}

impl FromUsize for (i32, i32) {
    fn from_usize(u: usize) -> (i32, i32) {
        (u as i32, 0)
    }
}

fn check_keyed<K, V, O, C>(mut c: C, expected_removed: O)
where
    K: FromUsize + Clone,
    V: FromUsize + Clone + PartialEq + Debug,
    O: PartialEq + Debug,
    C: Keyed<Key = K, Value = V>
        + Get<K>
        + Set<K>
        + Insert<K>
        + Remove<K, Output = O>
        + Clear
        + Len,
{
    let k1 = K::from_usize(1);
    let k2 = K::from_usize(2);
    let v1 = V::from_usize(10);
    let v2 = V::from_usize(20);

    assert_eq!(Len::len(&c), 0);
    assert_eq!(c.get(&k1), None);

    c.insert(k1.clone(), v1.clone());
    c.insert(k2.clone(), v2.clone());
    assert_eq!(Len::len(&c), 2);
    assert_eq!(c.get(&k1), Some(&v1));
    assert_eq!(c.get(&k2), Some(&v2));

    c.set(k1.clone(), v2.clone());
    assert_eq!(c.get(&k1), Some(&v2));

    assert_eq!(c.remove(&k1), expected_removed);
    assert_eq!(Len::len(&c), 1);
    assert_eq!(c.get(&k1), None);

    c.clear();
    assert_eq!(Len::len(&c), 0);
    assert_eq!(c.get(&k2), None);
}

fn check_entry<K, V, C>(mut c: C)
where
    K: FromUsize + Clone + PartialEq + Debug,
    V: FromUsize + Clone + PartialEq + Debug + Default,
    C: Keyed<Key = K, Value = V> + Entry<K> + Get<K> + Clear,
    for<'a> <C as Entry<K>>::Entry<'a>: CombinedEntry<'a, K, V>,
{
    let k = K::from_usize(1);
    let k2 = K::from_usize(2);

    assert_eq!(c.entry(k.clone()).key(), &k);

    *c.entry(k.clone()).or_insert(V::from_usize(10)) = V::from_usize(11);
    assert_eq!(c.get(&k), Some(&V::from_usize(11)));

    *c.entry(k.clone()).or_insert(V::from_usize(99)) = V::from_usize(12);
    assert_eq!(c.get(&k), Some(&V::from_usize(12)));

    *c.entry(k.clone()).or_insert_with(|| V::from_usize(13)) = V::from_usize(14);
    assert_eq!(c.get(&k), Some(&V::from_usize(14)));

    *c.entry(k2.clone()).or_insert_with_key(|key| {
        assert_eq!(key, &k2);
        V::from_usize(20)
    }) = V::from_usize(21);
    assert_eq!(c.get(&k2), Some(&V::from_usize(21)));

    c.entry(k.clone())
        .and_modify(|v| *v = V::from_usize(30))
        .or_insert(V::from_usize(31));
    assert_eq!(c.get(&k), Some(&V::from_usize(30)));

    c.clear();
    c.entry(k.clone())
        .and_modify(|v| *v = V::from_usize(40))
        .or_insert(V::from_usize(41));
    assert_eq!(c.get(&k), Some(&V::from_usize(41)));

    {
        let occupied = c.entry(k.clone()).insert_entry(V::from_usize(50));
        assert_eq!(OccupiedEntry::key(&occupied), &k);
        assert_eq!(OccupiedEntry::get(&occupied), &V::from_usize(50));
    }

    {
        let occupied = c.entry(k.clone()).insert_entry(V::from_usize(51));
        assert_eq!(OccupiedEntry::get(&occupied), &V::from_usize(51));
    }

    c.clear();
    assert_eq!(*c.entry(k.clone()).or_default(), V::default());
    assert_eq!(c.get(&k), Some(&V::default()));

    {
        let mut occupied = c.entry(k.clone()).insert_entry(V::from_usize(60));
        assert_eq!(
            OccupiedEntry::insert(&mut occupied, V::from_usize(61)),
            V::from_usize(60),
        );
        assert_eq!(OccupiedEntry::get(&occupied), &V::from_usize(61));

        *OccupiedEntry::get_mut(&mut occupied) = V::from_usize(62);
        assert_eq!(OccupiedEntry::get(&occupied), &V::from_usize(62));
    }

    c.clear();
    {
        let occupied = c.entry(k.clone()).insert_entry(V::from_usize(70));
        *OccupiedEntry::into_mut(occupied) = V::from_usize(71);
    }
    assert_eq!(c.get(&k), Some(&V::from_usize(71)));

    c.clear();
    {
        let occupied = c.entry(k.clone()).insert_entry(V::from_usize(80));
        assert_eq!(OccupiedEntry::remove(occupied), V::from_usize(80));
    }
    assert_eq!(c.get(&k), None);

    c.clear();
    {
        let occupied = c.entry(k.clone()).insert_entry(V::from_usize(90));
        assert_eq!(
            OccupiedEntry::remove_entry(occupied),
            (k.clone(), V::from_usize(90)),
        );
    }
    assert_eq!(c.get(&k), None);
}

fn check_modify<K, V, C>(mut c: C)
where
    K: FromUsize + Clone,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Keyed<Key = K, Value = V> + Insert<K> + Get<K> + Modify<K> + Clear,
{
    let k = K::from_usize(1);

    c.insert(k.clone(), V::from_usize(10));
    c.modify(&k, |v| *v = V::from_usize(99));
    assert_eq!(c.get(&k), Some(&V::from_usize(99)));

    c.clear();
    assert_eq!(c.get(&k), None);
}

fn check_into_iter<K, V, C>(mut c: C)
where
    K: FromUsize + Clone + PartialEq + Debug,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Keyed<Key = K, Value = V> + Insert<K> + IntoIter<K>,
{
    c.insert(K::from_usize(1), V::from_usize(10));
    c.insert(K::from_usize(2), V::from_usize(20));
    c.insert(K::from_usize(3), V::from_usize(30));

    let items: Vec<(K, V)> = IntoIter::into_iter(c).collect();

    assert_eq!(items.len(), 3);
    assert!(items.contains(&(K::from_usize(1), V::from_usize(10))));
    assert!(items.contains(&(K::from_usize(2), V::from_usize(20))));
    assert!(items.contains(&(K::from_usize(3), V::from_usize(30))));
}

fn check_assign<C>(initial: C, replacement: C)
where
    C: Assign + Clone + PartialEq + Debug,
{
    let mut c = initial;
    c.assign(replacement.clone());
    assert_eq!(c, replacement);
}

fn check_borrow_str<C>(mut c: C)
where
    C: Keyed<Key = String, Value = i32>
        + Insert<String>
        + Get<str>
        + Modify<str>
        + Remove<str, Output = Option<i32>>,
{
    c.insert("one".to_string(), 1);
    c.insert("two".to_string(), 2);

    assert_eq!(c.get("one"), Some(&1));
    assert_eq!(c.get("missing"), None);

    c.modify("two", |v| *v = 22);
    assert_eq!(c.get("two"), Some(&22));

    assert_eq!(c.remove("one"), Some(1));
    assert_eq!(c.get("one"), None);
}

fn check_push_put<K, V, C>(mut c: C)
where
    K: Clone,
    V: FromUsize + Clone + Ord + PartialEq + Debug,
    C: Keyed<Key = K, Value = V> + Push<K> + Get<K> + Set<K> + Modify<K> + Put<V> + IntoIter<K>,
{
    let k0 = c.push(V::from_usize(10));
    let k1 = c.push(V::from_usize(20));

    assert_eq!(c.get(&k0), Some(&V::from_usize(10)));
    assert_eq!(c.get(&k1), Some(&V::from_usize(20)));

    c.set(k0.clone(), V::from_usize(11));
    assert_eq!(c.get(&k0), Some(&V::from_usize(11)));

    c.modify(&k1, |v| *v = V::from_usize(21));
    assert_eq!(c.get(&k1), Some(&V::from_usize(21)));

    assert_eq!(c.put(V::from_usize(30)), None);

    let mut values: Vec<V> = IntoIter::into_iter(c).map(|(_, v)| v).collect();
    values.sort();
    assert_eq!(
        values,
        [V::from_usize(11), V::from_usize(21), V::from_usize(30),],
    );
}

fn check_with_one<E, V, C>(element: E, expected: V)
where
    V: PartialEq + Debug,
    C: Keyed<Value = V> + WithOne<E> + IntoIter<<C as Keyed>::Key>,
{
    let values: Vec<V> = IntoIter::into_iter(C::with_one(element))
        .map(|(_, value)| value)
        .collect();
    assert_eq!(values, [expected]);
}

fn check_pushed_insert_remove<K, V, C>(mut c: C)
where
    K: Clone,
    V: FromUsize + Clone + PartialEq + Debug,
    C: Keyed<Key = K, Value = V>
        + Push<K>
        + Get<K>
        + Insert<K>
        + Remove<K, Output = Option<V>>
        + Clear,
{
    let k0 = c.push(V::from_usize(10));

    assert_eq!(c.remove(&k0), Some(V::from_usize(10)));
    assert_eq!(c.get(&k0), None);

    c.insert(k0.clone(), V::from_usize(15));
    assert_eq!(c.get(&k0), Some(&V::from_usize(15)));

    c.clear();
    assert_eq!(c.get(&k0), None);
}

fn check_vec<V, C>(mut c: C)
where
    V: FromUsize + Clone + PartialEq + Debug,
    C: Keyed<Key = usize, Value = V> + Push<usize> + Pop + Len + Clear,
{
    c.push(V::from_usize(10));
    c.push(V::from_usize(20));
    c.push(V::from_usize(30));
    assert_eq!(Len::len(&c), 3);

    c.clear();
    assert_eq!(Len::len(&c), 0);
}

fn check_swap_remove<V, C>(mut c: C)
where
    V: FromUsize + Clone + PartialEq + Debug,
    C: Keyed<Key = usize, Value = V>
        + Push<usize>
        + Get<usize>
        + SwapRemove<usize, Output = V>
        + Len,
{
    c.push(V::from_usize(10));
    c.push(V::from_usize(20));
    c.push(V::from_usize(30));

    assert_eq!(c.swap_remove(&1), V::from_usize(20));
    assert_eq!(Len::len(&c), 2);
    assert_eq!(c.get(&0), Some(&V::from_usize(10)));
    assert_eq!(c.get(&1), Some(&V::from_usize(30)));

    assert_eq!(c.swap_remove(&1), V::from_usize(30));
    assert_eq!(Len::len(&c), 1);
    assert_eq!(c.get(&0), Some(&V::from_usize(10)));
}

fn check_resize<V, C>(mut c: C)
where
    V: FromUsize + Clone + PartialEq + Debug,
    C: Keyed<Key = usize, Value = V> + Get<usize> + Len + Resize,
{
    c.resize(3, V::from_usize(1));
    assert_eq!(Len::len(&c), 3);
    assert_eq!(c.get(&0), Some(&V::from_usize(1)));
    assert_eq!(c.get(&1), Some(&V::from_usize(1)));
    assert_eq!(c.get(&2), Some(&V::from_usize(1)));

    c.resize(5, V::from_usize(7));
    assert_eq!(Len::len(&c), 5);
    assert_eq!(c.get(&3), Some(&V::from_usize(7)));
    assert_eq!(c.get(&4), Some(&V::from_usize(7)));

    c.resize(2, V::from_usize(0));
    assert_eq!(Len::len(&c), 2);
}

fn check_indexed<C>(c: &mut C)
where
    C: ?Sized + Keyed<Key = usize, Value = i32> + Get<usize> + Set<usize> + Modify<usize> + Len,
{
    assert_eq!(Len::len(&*c), 3);

    assert_eq!(c.get(&0), Some(&10));

    c.set(1, 25);
    assert_eq!(c.get(&1), Some(&25));

    c.modify(&2, |v| *v += 5);
    assert_eq!(c.get(&2), Some(&35));
}

fn check_scalar<V>(initial: V, alt1: V, alt2: V, alt3: V)
where
    V: Clone + PartialEq + Debug,
    V: Keyed<Key = usize, Value = V>
        + Get<usize>
        + Set<usize, Output = Option<V>>
        + Modify<usize>
        + Put<V>
        + Len
        + IntoIter<usize>
        + WithOne<V>
        + Assign,
{
    let mut v = initial.clone();
    assert_eq!(v.get(&0), Some(&initial));
    assert_eq!(v.get(&1), None);
    assert_eq!(Len::len(&v), 1);

    assert_eq!(v.set(0, alt1.clone()), Some(initial.clone()));
    assert_eq!(v.get(&0), Some(&alt1));

    v.modify(&0, |x| *x = alt2.clone());
    assert_eq!(v.get(&0), Some(&alt2));

    assert_eq!(v.put(alt3.clone()), Some(alt2.clone()));
    assert_eq!(v.get(&0), Some(&alt3));

    assert_eq!(
        IntoIter::into_iter(v.clone()).collect::<Vec<_>>(),
        [(0, alt3.clone())],
    );

    check_with_one::<V, V, V>(alt1.clone(), alt1.clone());
    check_assign(v, alt1);
}

mod scalars_tests {
    use super::*;

    macro_rules! test_scalar {
        ($($t:ty: [$($v:expr),+]),* $(,)?) => {
            $(
                {
                    let [a, b, c, d]: [$t; 4] = [$($v),+];
                    check_scalar(a, b, c, d);
                }
            )*
        };
    }

    #[test]
    fn test_traits() {
        test_scalar! {
            i8: [10i8, 20, 30, 40],
            i16: [10i16, 20, 30, 40],
            i32: [10i32, 20, 30, 40],
            i64: [10i64, 20, 30, 40],
            i128: [10i128, 20, 30, 40],
            isize: [10isize, 20, 30, 40],
            u8: [10u8, 20, 30, 40],
            u16: [10u16, 20, 30, 40],
            u32: [10u32, 20, 30, 40],
            u64: [10u64, 20, 30, 40],
            u128: [10u128, 20, 30, 40],
            usize: [10, 20, 30, 40],
            f32: [1.0f32, 2.0, 3.0, 4.0],
            f64: [1.0f64, 2.0, 3.0, 4.0],
            char: ['a', 'b', 'c', 'd'],
            bool: [false, true, false, true],
            (): [(), (), (), ()],
        };
    }
}

mod one_tests {
    use super::*;
    use maplike::one::One;

    #[test]
    fn test_traits_on_one() {
        let mut c = One::new(10);

        assert_eq!(c.get(&0), Some(&10));
        assert_eq!(c.get(&1), None);

        assert_eq!(c.set(0, 20), Some(10));
        assert_eq!(c.get(&0), Some(&20));

        assert_eq!(c.put(30), Some(20));
        assert_eq!(c.get(&0), Some(&30));

        check_with_one::<i32, i32, One<i32>>(40, 40);
        check_assign(One::new(1), One::new(2));
    }
}

#[cfg(feature = "alloc")]
mod box_tests {
    use super::*;
    use std::boxed::Box;

    #[test]
    fn test_traits_on_box() {
        let mut c = Box::new(10);

        assert_eq!(c.get(&0), Some(&10));
        assert_eq!(c.get(&1), None);
        assert_eq!(c.len(), 1);

        assert_eq!(c.set(0, 20), Some(10));
        assert_eq!(c.get(&0), Some(&20));

        c.modify(&0, |v| *v = 21);
        assert_eq!(c.get(&0), Some(&21));

        assert_eq!(c.put(30), Some(21));
        assert_eq!(c.get(&0), Some(&30));

        check_with_one::<i32, i32, Box<i32>>(40, 40);
        check_assign(Box::new(1), Box::new(2));

        assert_eq!(
            IntoIter::into_iter(Box::new(9)).collect::<Vec<_>>(),
            [(0, 9)],
        );
    }
}

#[cfg(feature = "alloc")]
mod rc_tests {
    use super::*;
    use std::rc::{Rc, Weak};

    #[test]
    fn test_traits_on_rc() {
        let mut c = Rc::new(10);

        assert_eq!(c.get(&0), Some(&10));
        assert_eq!(c.get(&1), None);
        assert_eq!(c.len(), 1);

        assert_eq!(c.set(0, 20), Some(10));
        assert_eq!(c.get(&0), Some(&20));

        c.modify(&0, |v| *v = 21);
        assert_eq!(c.get(&0), Some(&21));

        assert_eq!(c.put(30), Some(21));
        assert_eq!(c.get(&0), Some(&30));

        let c = Rc::with_one(40);
        assert_eq!(c.get(&0), Some(&40));
        assert_eq!(c.len(), 1);
        check_assign(Rc::new(1), Rc::new(2));
    }

    #[test]
    fn test_traits_on_rc_weak() {
        let rc = Rc::new(42);
        let mut weak = Rc::downgrade(&rc);
        assert_eq!(weak.len(), 1);

        assert_eq!(weak.remove(&0), None);
        assert_eq!(weak.len(), 0);

        let rc = Rc::new(42);
        let weak = Rc::downgrade(&rc);
        drop(rc);
        assert_eq!(weak.len(), 0);

        let rc = Rc::new(42);
        let mut weak = Rc::downgrade(&rc);
        weak.clear();
        assert_eq!(weak.len(), 0);

        /*let rc = Rc::new(9);
        let weak = Rc::downgrade(&rc);
        assert_eq!(IntoIter::into_iter(weak).collect::<Vec<_>>(), [(0, 9)],);

        let rc = Rc::new(9);
        let weak = Rc::downgrade(&rc);
        drop(rc);
        assert_eq!(IntoIter::into_iter(weak).collect::<Vec<_>>(), Vec::new());*/

        let mut weak = Weak::<i32>::new();
        let rc = Rc::new(1);
        weak.assign(Rc::downgrade(&rc));
        assert_eq!(weak.len(), 1);
    }
}

#[cfg(feature = "std")]
mod std_tests {
    use super::*;
    use std::collections::hash_map::Entry as HashMapStdEntry;
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, Weak};

    fn check_hashmap_entry_variants() {
        let mut map = HashMap::<usize, i32>::new();
        if let HashMapStdEntry::Vacant(v) = map.entry(1) {
            assert_eq!(VacantEntry::key(&v), &1);
        } else {
            panic!("expected vacant entry");
        }

        let mut map = HashMap::<usize, i32>::new();
        if let HashMapStdEntry::Vacant(v) = map.entry(1) {
            assert_eq!(VacantEntry::into_key(v), 1);
        } else {
            panic!("expected vacant entry");
        }

        let mut map = HashMap::<usize, i32>::new();
        if let HashMapStdEntry::Vacant(v) = map.entry(2) {
            assert_eq!(*VacantEntry::insert(v, 20), 20);
        } else {
            panic!("expected vacant entry");
        }
        assert_eq!(map.get(&2), Some(&20));

        let mut map = HashMap::<usize, i32>::new();
        if let HashMapStdEntry::Vacant(v) = map.entry(3) {
            let occupied = VacantEntry::insert_entry(v, 30);
            assert_eq!(OccupiedEntry::get(&occupied), &30);
        } else {
            panic!("expected vacant entry");
        }
        assert_eq!(map.get(&3), Some(&30));

        let mut map = HashMap::<usize, i32>::new();
        map.entry(4).or_insert(40);
        if let HashMapStdEntry::Occupied(o) = map.entry(4) {
            assert_eq!(OccupiedEntry::key(&o), &4);
            assert_eq!(OccupiedEntry::get(&o), &40);
        } else {
            panic!("expected occupied entry");
        }
    }

    #[test]
    fn test_traits_on_hashmap() {
        check_keyed(HashMap::<usize, i32>::new(), Some(20));
        check_modify::<usize, i32, HashMap<usize, i32>>(HashMap::new());
        check_into_iter::<usize, i32, HashMap<usize, i32>>(HashMap::new());
        check_borrow_str(HashMap::<String, i32>::new());
        check_entry::<usize, i32, HashMap<usize, i32>>(HashMap::new());
        check_hashmap_entry_variants();
        check_assign(
            HashMap::from([(1, 1i32)]),
            HashMap::from([(2, 2i32), (3, 3i32)]),
        );
    }

    #[test]
    fn test_traits_on_hashset() {
        check_keyed(HashSet::<usize>::new(), Some(()));
        check_into_iter::<usize, (), HashSet<usize>>(HashSet::new());
        check_with_one::<usize, (), HashSet<usize>>(7, ());
        check_assign(HashSet::from([1]), HashSet::from([2, 3]));
    }

    #[test]
    fn test_traits_on_arc() {
        let mut c = Arc::new(10);

        assert_eq!(c.get(&0), Some(&10));
        assert_eq!(c.get(&1), None);
        assert_eq!(c.len(), 1);

        assert_eq!(c.set(0, 20), Some(10));
        assert_eq!(c.get(&0), Some(&20));

        c.modify(&0, |v| *v = 21);
        assert_eq!(c.get(&0), Some(&21));

        assert_eq!(c.put(30), Some(21));
        assert_eq!(c.get(&0), Some(&30));

        let c = Arc::with_one(40);
        assert_eq!(c.get(&0), Some(&40));
        assert_eq!(c.len(), 1);
        check_assign(Arc::new(1), Arc::new(2));
    }

    #[test]
    fn test_traits_on_arc_weak() {
        let arc = Arc::new(42);
        let mut weak = Arc::downgrade(&arc);
        assert_eq!(weak.len(), 1);

        assert_eq!(weak.remove(&0), None);
        assert_eq!(weak.len(), 0);

        let arc = Arc::new(42);
        let weak = Arc::downgrade(&arc);
        drop(arc);
        assert_eq!(weak.len(), 0);

        /*let arc = Arc::new(9);
        let weak = Arc::downgrade(&arc);
        assert_eq!(IntoIter::into_iter(weak).collect::<Vec<_>>(), [(0, 9)],);

        let arc = Arc::new(9);
        let weak = Arc::downgrade(&arc);
        drop(arc);
        assert_eq!(IntoIter::into_iter(weak).collect::<Vec<_>>(), Vec::new());*/

        let mut weak = Weak::<i32>::new();
        let arc = Arc::new(1);
        weak.assign(Arc::downgrade(&arc));
        assert_eq!(weak.len(), 1);
    }
}

#[cfg(feature = "alloc")]
mod alloc_tests {
    use super::*;
    use std::collections::btree_map::Entry as BTreeMapStdEntry;
    use std::collections::{BTreeMap, BTreeSet};

    fn check_btreemap_entry_variants() {
        let mut map = BTreeMap::<usize, i32>::new();
        if let BTreeMapStdEntry::Vacant(v) = map.entry(1) {
            assert_eq!(VacantEntry::key(&v), &1);
        } else {
            panic!("expected vacant entry");
        }

        let mut map = BTreeMap::<usize, i32>::new();
        if let BTreeMapStdEntry::Vacant(v) = map.entry(1) {
            assert_eq!(VacantEntry::into_key(v), 1);
        } else {
            panic!("expected vacant entry");
        }

        let mut map = BTreeMap::<usize, i32>::new();
        if let BTreeMapStdEntry::Vacant(v) = map.entry(2) {
            assert_eq!(*VacantEntry::insert(v, 20), 20);
        } else {
            panic!("expected vacant entry");
        }
        assert_eq!(map.get(&2), Some(&20));

        let mut map = BTreeMap::<usize, i32>::new();
        if let BTreeMapStdEntry::Vacant(v) = map.entry(3) {
            let occupied = VacantEntry::insert_entry(v, 30);
            assert_eq!(OccupiedEntry::get(&occupied), &30);
        } else {
            panic!("expected vacant entry");
        }
        assert_eq!(map.get(&3), Some(&30));

        let mut map = BTreeMap::<usize, i32>::new();
        map.entry(4).or_insert(40);
        if let BTreeMapStdEntry::Occupied(o) = map.entry(4) {
            assert_eq!(OccupiedEntry::key(&o), &4);
            assert_eq!(OccupiedEntry::get(&o), &40);
        } else {
            panic!("expected occupied entry");
        }
    }

    #[test]
    fn test_traits_on_btreemap() {
        check_keyed(BTreeMap::<usize, i32>::new(), Some(20));
        check_modify::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
        check_into_iter::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
        check_borrow_str(BTreeMap::<String, i32>::new());
        check_entry::<usize, i32, BTreeMap<usize, i32>>(BTreeMap::new());
        check_btreemap_entry_variants();
        check_assign(
            BTreeMap::from([(1, 1i32)]),
            BTreeMap::from([(2, 2i32), (3, 3i32)]),
        );
    }

    #[test]
    fn test_traits_on_btreeset() {
        check_keyed(BTreeSet::<usize>::new(), Some(()));
        check_into_iter::<usize, (), BTreeSet<usize>>(BTreeSet::new());
        check_with_one::<usize, (), BTreeSet<usize>>(7, ());
        check_assign(BTreeSet::from([1]), BTreeSet::from([2, 3]));
    }

    #[test]
    fn test_traits_on_vec() {
        check_push_put::<usize, i32, Vec<i32>>(Vec::new());
        check_with_one::<i32, i32, Vec<i32>>(30, 30);
        check_vec::<i32, Vec<i32>>(Vec::new());
        check_swap_remove::<i32, Vec<i32>>(Vec::new());
        check_resize::<i32, Vec<i32>>(Vec::new());
        check_assign(vec![1i32], vec![2i32, 3i32]);

        let mut c: Vec<i32> = Vec::new();
        c.push(10);
        c.push(20);
        c.push(30);
        let items: Vec<(usize, i32)> = IntoIter::into_iter(c).collect();
        assert_eq!(items, vec![(0, 10), (1, 20), (2, 30)]);
    }

    #[test]
    fn test_traits_on_vecdeque() {
        use std::collections::VecDeque;

        check_push_put::<usize, i32, VecDeque<i32>>(VecDeque::new());
        check_with_one::<i32, i32, VecDeque<i32>>(30, 30);
        check_vec::<i32, VecDeque<i32>>(VecDeque::new());
        check_resize::<i32, VecDeque<i32>>(VecDeque::new());
        check_assign(VecDeque::from([1i32]), VecDeque::from([2i32, 3i32]));

        let mut c: VecDeque<i32> = VecDeque::new();
        c.push(10);
        c.push(20);
        c.push(30);
        let items: Vec<(usize, i32)> = IntoIter::into_iter(c).collect();
        assert_eq!(items, vec![(0, 10), (1, 20), (2, 30)]);
    }
}

mod array_tests {
    use super::*;

    #[test]
    fn test_traits_on_array() {
        check_indexed(&mut [10i32, 20, 30]);
        check_assign([0i32, 0, 0], [1i32, 2, 3]);
    }
}

mod slice_tests {
    use super::*;

    #[test]
    fn test_traits_on_slice() {
        let mut backing = [10i32, 20, 30];
        check_indexed(&mut backing[..]);
    }
}

mod tuple_tests {
    use super::*;

    #[test]
    fn test_traits_on_tuple() {
        check_assign((0i32, 0i32), (1i32, 2i32));
    }
}

mod option_tests {
    use super::*;

    #[test]
    fn test_traits_on_option() {
        let mut c = None;
        assert_eq!(c.get(&0), None);
        assert_eq!(c.get(&1), None);
        assert_eq!(c.len(), 0);

        assert_eq!(c.put(i32::from_usize(10)), None);
        assert_eq!(c.get(&0), Some(&i32::from_usize(10)));
        assert_eq!(c.len(), 1);

        assert_eq!(c.set(0, i32::from_usize(11)), Some(i32::from_usize(10)));
        assert_eq!(c.get(&0), Some(&i32::from_usize(11)));

        c.modify(&0, |v| *v = i32::from_usize(12));
        assert_eq!(c.get(&0), Some(&i32::from_usize(12)));

        assert_eq!(c.put(i32::from_usize(13)), Some(i32::from_usize(12)));
        assert_eq!(c.put(i32::from_usize(14)), Some(i32::from_usize(13)));
        assert_eq!(c.get(&0), Some(&i32::from_usize(14)));

        assert_eq!(c.remove(&1), None);
        assert_eq!(c.remove(&0), Some(i32::from_usize(14)));
        assert_eq!(c.len(), 0);

        c.clear();
        assert_eq!(c.get(&0), None);

        assert_eq!(Option::<i32>::with_one(5), Some(5));
        assert_eq!(
            IntoIter::into_iter(Option::<i32>::with_one(9)).collect::<Vec<_>>(),
            [(0, 9)],
        );

        check_with_one::<i32, i32, Option<i32>>(30, 30);
    }
}

#[cfg(feature = "stable-vec")]
mod stable_vec_tests {
    use super::*;
    use stable_vec::StableVec;

    #[test]
    fn test_traits_on_stable_vec() {
        check_keyed(StableVec::<i32>::new(), Some(20));
        check_modify::<usize, i32, StableVec<i32>>(StableVec::new());
        check_into_iter::<usize, i32, StableVec<i32>>(StableVec::new());
        check_push_put::<usize, i32, StableVec<i32>>(StableVec::new());
        check_with_one::<i32, i32, StableVec<i32>>(30, 30);
        check_pushed_insert_remove::<usize, i32, StableVec<i32>>(StableVec::new());

        let mut a = StableVec::new();
        a.push(1i32);
        let mut b = StableVec::new();
        b.push(2i32);
        b.push(3i32);
        check_assign(a, b);
    }
}

#[cfg(feature = "slab")]
mod slab_tests {
    use super::*;
    use slab::Slab;

    #[test]
    fn test_traits_on_slab() {
        check_push_put::<usize, i32, Slab<i32>>(Slab::new());
        check_with_one::<i32, i32, Slab<i32>>(30, 30);

        let mut a: Slab<i32> = Slab::new();
        assert_eq!(Len::len(&a), 0);
        let k0 = a.push(1);
        let k1 = a.push(2);
        let k2 = a.push(3);
        assert_eq!(Len::len(&a), 3);
        assert_eq!(Get::get(&a, &k0), Some(&1));
        assert_eq!(Remove::remove(&mut a, &k1), Some(2));
        assert_eq!(Get::get(&a, &k1), None);
        assert_eq!(Len::len(&a), 2);

        let items: Vec<(usize, i32)> = IntoIter::into_iter(a).collect();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&(k0, 1)));
        assert!(items.contains(&(k2, 3)));

        let mut x: Slab<i32> = Slab::new();
        x.push(5);
        let mut y: Slab<i32> = Slab::new();
        let j = y.push(7);
        x.assign(y);
        assert_eq!(Get::get(&x, &j), Some(&7));
        assert_eq!(Len::len(&x), 1);
    }
}

#[cfg(feature = "thunderdome")]
mod thunderdome_tests {
    use super::*;
    use thunderdome::Arena;

    #[test]
    fn test_traits_on_arena() {
        check_push_put::<thunderdome::Index, i32, Arena<i32>>(Arena::new());
        check_with_one::<i32, i32, Arena<i32>>(30, 30);
        check_pushed_insert_remove::<thunderdome::Index, i32, Arena<i32>>(Arena::new());

        let mut a: Arena<i32> = Arena::new();
        assert_eq!(Len::len(&a), 0);

        a.push(1);
        a.push(2);
        a.push(3);
        assert_eq!(Len::len(&a), 3);

        let items: Vec<(thunderdome::Index, i32)> = IntoIter::into_iter(a).collect();
        assert_eq!(items.len(), 3);

        let mut x: Arena<i32> = Arena::new();
        x.push(5);
        let mut y: Arena<i32> = Arena::new();
        let j = y.push(7);
        x.assign(y);
        assert_eq!(Get::get(&x, &j), Some(&7));
        assert_eq!(Len::len(&x), 1);
    }
}

#[cfg(feature = "arrayvec")]
mod arrayvec_tests {
    use super::*;
    use arrayvec::{ArrayString, ArrayVec};

    #[test]
    fn test_traits_on_arrayvec() {
        check_push_put::<usize, i32, ArrayVec<i32, 8>>(ArrayVec::new());
        check_with_one::<i32, i32, ArrayVec<i32, 8>>(30, 30);
        check_vec::<i32, ArrayVec<i32, 8>>(ArrayVec::new());
        check_assign(
            {
                let mut a = ArrayVec::<i32, 8>::new();
                a.push(1);
                a
            },
            {
                let mut b = ArrayVec::<i32, 8>::new();
                b.push(2);
                b.push(3);
                b
            },
        );
    }

    #[test]
    fn test_traits_on_arraystring() {
        let mut s: ArrayString<8> = ArrayString::new();
        assert_eq!(Push::push(&mut s, 'a'), 0);
        assert_eq!(Push::push(&mut s, 'b'), 1);
        assert_eq!(Len::len(&s), 2);
        assert_eq!(s.as_str(), "ab");
        assert_eq!(Pop::pop(&mut s), Some('b'));
        assert_eq!(Pop::pop(&mut s), Some('a'));
        assert_eq!(Pop::pop(&mut s), None);

        let one: ArrayString<8> = WithOne::with_one('x');
        assert_eq!(one.as_str(), "x");
        //let items: Vec<(usize, char)> = IntoIter::into_iter(one).collect();
        //assert_eq!(items, [(0, 'x')]);

        Clear::clear(&mut s);
        assert_eq!(Len::len(&s), 0);

        let a = ArrayString::<8>::from("hi").unwrap();
        let b = ArrayString::<8>::from("bye").unwrap();
        check_assign(a, b);
    }
}

#[cfg(feature = "smallvec")]
mod smallvec_tests {
    use super::*;
    use smallvec::SmallVec;

    #[test]
    fn test_traits_on_smallvec() {
        check_push_put::<usize, i32, SmallVec<[i32; 8]>>(SmallVec::new());
        check_with_one::<i32, i32, SmallVec<[i32; 8]>>(30, 30);
        check_vec::<i32, SmallVec<[i32; 8]>>(SmallVec::new());
        check_resize::<i32, SmallVec<[i32; 8]>>(SmallVec::new());
        check_assign(
            {
                let mut a = SmallVec::<[i32; 8]>::new();
                a.push(1);
                a
            },
            {
                let mut b = SmallVec::<[i32; 8]>::new();
                b.push(2);
                b.push(3);
                b
            },
        );
    }
}

#[cfg(feature = "tinyvec")]
mod tinyvec_tests {
    use super::*;
    use tinyvec::{ArrayVec, TinyVec};

    #[test]
    fn test_traits_on_arrayvec() {
        check_push_put::<usize, i32, ArrayVec<[i32; 8]>>(ArrayVec::new());
        check_with_one::<i32, i32, ArrayVec<[i32; 8]>>(30, 30);
        check_vec::<i32, ArrayVec<[i32; 8]>>(ArrayVec::new());
        check_resize::<i32, ArrayVec<[i32; 8]>>(ArrayVec::new());
        check_assign(
            ArrayVec::from_array_len([1i32, 0, 0, 0, 0, 0, 0, 0], 1),
            ArrayVec::from_array_len([2i32, 3, 0, 0, 0, 0, 0, 0], 2),
        );
    }

    #[test]
    fn test_traits_on_tinyvec() {
        check_push_put::<usize, i32, TinyVec<[i32; 8]>>(TinyVec::new());
        check_with_one::<i32, i32, TinyVec<[i32; 8]>>(30, 30);
        check_vec::<i32, TinyVec<[i32; 8]>>(TinyVec::new());
        check_resize::<i32, TinyVec<[i32; 8]>>(TinyVec::new());

        let mut a = TinyVec::<[i32; 8]>::new();
        a.push(1);
        let mut b = TinyVec::<[i32; 8]>::new();
        b.push(2);
        b.push(3);
        check_assign(a, b);
    }
}

#[cfg(feature = "rstar")]
mod rstar_tests {
    use super::*;
    use rstar::RTree;

    #[test]
    fn test_traits_on_rtree() {
        check_keyed(RTree::<(i32, i32)>::new(), Some(()));
        check_into_iter::<(i32, i32), (), RTree<(i32, i32)>>(RTree::new());
        check_with_one::<(i32, i32), (), RTree<(i32, i32)>>((3, 4), ());

        let mut r: RTree<(i32, i32)> = RTree::new();
        Insert::insert(&mut r, (1, 0), ());
        let mut s: RTree<(i32, i32)> = RTree::new();
        Insert::insert(&mut s, (2, 0), ());
        r.assign(s);
        assert!(Get::get(&r, &(2, 0)).is_some());
        assert!(Get::get(&r, &(1, 0)).is_none());
    }
}

#[cfg(feature = "indexmap")]
mod indexmap_tests {
    use super::*;
    use indexmap::map::Entry as IndexMapStdEntry;
    use indexmap::{IndexMap, IndexSet};

    fn check_indexmap_entry_variants() {
        let mut map = IndexMap::<usize, i32>::new();
        let IndexMapStdEntry::Vacant(v) = map.entry(1) else {
            panic!("expected vacant entry");
        };
        assert_eq!(VacantEntry::key(&v), &1);

        let mut map = IndexMap::<usize, i32>::new();
        let IndexMapStdEntry::Vacant(v) = map.entry(1) else {
            panic!("expected vacant entry");
        };
        assert_eq!(VacantEntry::into_key(v), 1);

        let mut map = IndexMap::<usize, i32>::new();
        let IndexMapStdEntry::Vacant(v) = map.entry(2) else {
            panic!("expected vacant entry");
        };
        assert_eq!(*VacantEntry::insert(v, 20), 20);
        assert_eq!(map.get(&2), Some(&20));

        let mut map = IndexMap::<usize, i32>::new();
        let IndexMapStdEntry::Vacant(v) = map.entry(3) else {
            panic!("expected vacant entry");
        };
        let occupied = VacantEntry::insert_entry(v, 30);
        assert_eq!(OccupiedEntry::get(&occupied), &30);
        assert_eq!(map.get(&3), Some(&30));

        let mut map = IndexMap::<usize, i32>::new();
        map.entry(4).or_insert(40);
        let IndexMapStdEntry::Occupied(o) = map.entry(4) else {
            panic!("expected occupied entry");
        };
        assert_eq!(OccupiedEntry::key(&o), &4);
        assert_eq!(OccupiedEntry::get(&o), &40);
    }

    #[test]
    fn test_traits_on_indexmap() {
        check_keyed(IndexMap::<usize, i32>::new(), Some(20));
        check_modify::<usize, i32, IndexMap<usize, i32>>(IndexMap::new());
        check_into_iter::<usize, i32, IndexMap<usize, i32>>(IndexMap::new());
        check_borrow_str(IndexMap::<String, i32>::new());
        check_entry::<usize, i32, IndexMap<usize, i32>>(IndexMap::new());
        check_indexmap_entry_variants();
        check_assign(
            IndexMap::from([(1, 1i32)]),
            IndexMap::from([(2, 2i32), (3, 3i32)]),
        );
    }

    #[test]
    fn test_traits_on_indexset() {
        check_keyed(IndexSet::<usize>::new(), Some(()));
        check_into_iter::<usize, (), IndexSet<usize>>(IndexSet::new());
        check_with_one::<usize, (), IndexSet<usize>>(7, ());
        check_assign(IndexSet::from([1]), IndexSet::from([2, 3]));
    }
}

#[cfg(feature = "bidimap")]
mod bidimap_tests {
    use super::*;
    #[cfg(feature = "std")]
    use bidimap::BiHashMap;
    use bidimap::{BiBTreeMap, Overwritten};

    fn check_bidirectional_map<C>(mut c: C)
    where
        C: Keyed<Key = String, Value = i32>
            + Get<String>
            + GetByLeft<str>
            + GetByRight<String>
            + Set<String, Output = Overwritten<String, i32>>
            + Insert<String, Output = Overwritten<String, i32>>
            + RemoveByLeft<str>
            + RemoveByRight<String>
            + Clear
            + Len,
    {
        assert_eq!(Len::len(&c), 0);
        assert_eq!(c.insert("a".to_string(), 1), Overwritten::Neither);
        assert_eq!(c.insert("b".to_string(), 2), Overwritten::Neither);
        assert_eq!(Len::len(&c), 2);

        assert_eq!(c.get(&"a".to_string()), Some(&1));
        assert_eq!(c.get_by_left("a"), Some(&1));
        assert_eq!(c.get_by_right(&2), Some(&"b".to_string()));

        assert_eq!(
            c.set("a".to_string(), 11),
            Overwritten::Left("a".to_string(), 1)
        );
        assert_eq!(c.get_by_left("a"), Some(&11));
        assert_eq!(c.get_by_right(&11), Some(&"a".to_string()));

        assert_eq!(c.remove_by_left("a"), Some(11));
        assert_eq!(Len::len(&c), 1);
        assert_eq!(c.get_by_left("a"), None);

        assert_eq!(c.remove_by_right(&2), Some("b".to_string()));
        assert_eq!(Len::len(&c), 0);
        c.clear();
        assert_eq!(c.get_by_left("b"), None);
    }

    #[test]
    fn test_traits_on_bibtreemap() {
        check_bidirectional_map(BiBTreeMap::<String, i32>::new());
        check_into_iter::<usize, i32, BiBTreeMap<usize, i32>>(BiBTreeMap::new());

        let mut a = BiBTreeMap::new();
        a.insert("a".to_string(), 1i32);
        let mut b = BiBTreeMap::new();
        b.insert("b".to_string(), 2i32);
        check_assign(a, b);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_traits_on_bihashmap() {
        check_bidirectional_map(BiHashMap::<String, i32>::new());
        check_into_iter::<usize, i32, BiHashMap<usize, i32>>(BiHashMap::new());

        let mut a = BiHashMap::new();
        a.insert("a".to_string(), 1i32);
        let mut b = BiHashMap::new();
        b.insert("b".to_string(), 2i32);
        check_assign(a, b);
    }
}

#[cfg(feature = "geo")]
mod geo_tests {
    use super::*;
    use geo_types::{
        Coord, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
        MultiPolygon, Point, Polygon, Rect, Triangle, coord, line_string,
    };

    impl FromUsize for Coord<i32> {
        fn from_usize(u: usize) -> Self {
            Coord { x: u as i32, y: 0 }
        }
    }

    impl FromUsize for Point<i32> {
        fn from_usize(u: usize) -> Self {
            Point::new(u as i32, 0)
        }
    }

    impl FromUsize for LineString<i32> {
        fn from_usize(u: usize) -> Self {
            let x = u as i32;
            line_string![(x: x, y: 0), (x: x + 1, y: 0)]
        }
    }

    impl FromUsize for Polygon<i32> {
        fn from_usize(u: usize) -> Self {
            let x = u as i32;
            Polygon::new(
                line_string![
                    (x: x, y: x),
                    (x: x + 1, y: x),
                    (x: x + 1, y: x + 1),
                    (x: x, y: x),
                ],
                vec![],
            )
        }
    }

    impl FromUsize for Geometry<i32> {
        fn from_usize(u: usize) -> Self {
            Geometry::Point(Point::from_usize(u))
        }
    }

    #[test]
    fn test_traits_on_geo_scalars() {
        check_scalar(
            coord! { x: 1, y: 2 },
            coord! { x: 3, y: 4 },
            coord! { x: 5, y: 6 },
            coord! { x: 7, y: 8 },
        );
        check_scalar(
            Point::new(1, 2),
            Point::new(3, 4),
            Point::new(5, 6),
            Point::new(7, 8),
        );
        check_scalar(
            Line::new(coord! { x: 0, y: 0 }, coord! { x: 1, y: 1 }),
            Line::new(coord! { x: 2, y: 2 }, coord! { x: 3, y: 3 }),
            Line::new(coord! { x: 4, y: 4 }, coord! { x: 5, y: 5 }),
            Line::new(coord! { x: 6, y: 6 }, coord! { x: 7, y: 7 }),
        );
        check_scalar(
            Rect::new(coord! { x: 0, y: 0 }, coord! { x: 1, y: 1 }),
            Rect::new(coord! { x: 2, y: 2 }, coord! { x: 3, y: 3 }),
            Rect::new(coord! { x: 4, y: 4 }, coord! { x: 5, y: 5 }),
            Rect::new(coord! { x: 6, y: 6 }, coord! { x: 7, y: 7 }),
        );
        check_scalar(
            Triangle::new(
                coord! { x: 0, y: 0 },
                coord! { x: 1, y: 0 },
                coord! { x: 0, y: 1 },
            ),
            Triangle::new(
                coord! { x: 2, y: 2 },
                coord! { x: 3, y: 2 },
                coord! { x: 2, y: 3 },
            ),
            Triangle::new(
                coord! { x: 4, y: 4 },
                coord! { x: 5, y: 4 },
                coord! { x: 4, y: 5 },
            ),
            Triangle::new(
                coord! { x: 6, y: 6 },
                coord! { x: 7, y: 6 },
                coord! { x: 6, y: 7 },
            ),
        );

        let poly = |x: i32| {
            Polygon::new(
                line_string![
                    (x: x, y: x),
                    (x: x + 1, y: x),
                    (x: x + 1, y: x + 1),
                    (x: x, y: x),
                ],
                vec![],
            )
        };
        check_scalar(poly(0), poly(2), poly(4), poly(6));
        check_scalar(
            Geometry::Point(Point::new(1, 2)),
            Geometry::Point(Point::new(3, 4)),
            Geometry::Point(Point::new(5, 6)),
            Geometry::Point(Point::new(7, 8)),
        );
    }

    #[test]
    fn test_traits_on_geo_veclike() {
        check_with_one::<Coord<i32>, Coord<i32>, LineString<i32>>(
            coord! { x: 30, y: 0 },
            coord! { x: 30, y: 0 },
        );
        check_vec::<Coord<i32>, LineString<i32>>(LineString::new(vec![]));
        check_resize::<Coord<i32>, LineString<i32>>(LineString::new(vec![]));
        check_assign(
            LineString::new(vec![coord! { x: 1, y: 0 }]),
            LineString::new(vec![coord! { x: 2, y: 0 }, coord! { x: 3, y: 0 }]),
        );

        check_with_one::<Point<i32>, Point<i32>, MultiPoint<i32>>(
            Point::new(30, 0),
            Point::new(30, 0),
        );
        check_vec::<Point<i32>, MultiPoint<i32>>(MultiPoint::new(vec![]));
        check_assign(
            MultiPoint::new(vec![Point::new(1, 0)]),
            MultiPoint::new(vec![Point::new(2, 0), Point::new(3, 0)]),
        );

        let ls = |x: i32| line_string![(x: x, y: 0), (x: x + 1, y: 0)];
        check_with_one::<LineString<i32>, LineString<i32>, MultiLineString<i32>>(ls(30), ls(30));
        check_vec::<LineString<i32>, MultiLineString<i32>>(MultiLineString::new(vec![]));

        let poly = |x: i32| {
            Polygon::new(
                line_string![
                    (x: x, y: x),
                    (x: x + 1, y: x),
                    (x: x + 1, y: x + 1),
                    (x: x, y: x),
                ],
                vec![],
            )
        };
        check_with_one::<Polygon<i32>, Polygon<i32>, MultiPolygon<i32>>(poly(30), poly(30));
        check_vec::<Polygon<i32>, MultiPolygon<i32>>(MultiPolygon::new(vec![]));

        check_with_one::<Geometry<i32>, Geometry<i32>, GeometryCollection<i32>>(
            Geometry::Point(Point::new(30, 0)),
            Geometry::Point(Point::new(30, 0)),
        );
        check_vec::<Geometry<i32>, GeometryCollection<i32>>(GeometryCollection::new_from(vec![]));
    }
}
