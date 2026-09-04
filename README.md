<!--
SPDX-FileCopyrightText: 2026 maplike contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Repository](https://img.shields.io/badge/repository-GitHub-0FBF3E)](https://github.com/mikwielgus/maplike)
[![Docs](https://docs.rs/maplike/badge.svg)](https://docs.rs/maplike/)
[![Crates.io](https://img.shields.io/crates/v/maplike.svg)](https://crates.io/crates/maplike)
[![MSRV](https://img.shields.io/crates/msrv/maplike.svg)](https://blog.rust-lang.org/2025/12/11/Rust-1.92.0/)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/maplike.svg)](#licence)

# maplike

Rust traits for abstract containers and operations over them.

With this library, you can write code that is generic over various built-in,
standard library, and third-party collections, containers, and primitives. You
can have the same code work on both on `BTreeMap` and `HashMap`, and in many
cases also on `Vec`, `Option`, `Box`, `Rc`, and more.

See the [Supported containers](#supported-containers) section for a complete
list of supported containers.

Basically, this is Python's
[collections.abc](https://docs.python.org/3/library/collections.abc.html), but
in Rust, and with traits not only for different kinds of containers, but also
for each operation. Essentially, every container is treated as if it was a
map. If it is not really a map, then it is treated as if its key type was
`usize`, even when there can be at most only one element. Hence the crate name,
`maplike`.

The traits are implemented for many containers from `std` and third-party
crates. See the [Traits](#traits) section for a list of all available traits.

This library is maintained and champaigned (aka.
[dogfooded](https://en.wikipedia.org/wiki/Eating_your_own_dog_food)) by the
authors, who use has it as a dependency for
- [`undoredo`](https://github.com/mikwielgus/undoredo), a versatile crate for
implementing Undo/Redo and non-linear history tree using sparse deltas (diffs),
snapshots, or commands on arbitrary data structures;
- [`multi_bimap`](https://github.com/mikwielgus/multi_bimap), a crate
implementing many-to-many bidirectional map using two antiparallel internal
containers chosen by the user;
- [`dcel`](https://github.com/mikwielgus/dcel), a crate that implements the
half-edge data structure (aka. doubly connected edge list, DCEL) generically
over its underlying containers.

This crate is compatible with `no_std` and `serde` and contains no `unsafe`
code.

If you are looking for abstract number traits instead of or in addition
to abstract container traits, also check out another crate of ours,
[`numlike`](https://github.com/mikwielgus/numlike).

## Usage

### Adding dependency

First, add `maplike` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
maplike = { version = "0.14.0", features = ["derive"] }
```

The `derive` feature flag is only needed if you want to
derive the `Keyed` trait using derive macro
[`#[derive(Keyed)]`](https://docs.rs/maplike/latest/maplike/derive.Keyed.html).

### Usage examples

`maplike`'s traits allow you to write functions that are generic over many
different collection types. A single trait like
[`Get`](https://docs.rs/maplike/latest/maplike/ops/trait.Get.html) is enough to
abstract over `Vec`s, arrays, and maps alike.

```rust
use std::collections::{BTreeMap, HashMap};

use maplike::ops::Get;

// Generic over any collection implementing the `Get` trait.
fn get_second_element<C: Get<usize>>(collection: &C) -> Option<&C::Value> {
    collection.get(&1)
}

// `get_second_element()` works for `Vec`s, arrays, `BTreeMap`s, `HashMap`s with
// the very same code.
assert_eq!(get_second_element(&vec![10, 20, 30]), Some(&20));
assert_eq!(get_second_element(&[10, 20, 30]), Some(&20));
assert_eq!(get_second_element(&BTreeMap::from([(0, 10), (1, 20)])), Some(&20));
assert_eq!(get_second_element(&HashMap::from([(0, 10), (1, 20)])), Some(&20));
```

An abstract container trait can bundle together several traits
for container methods together in one short bound. For example,
[`Veclike`](https://docs.rs/maplike/latest/maplike/abc/trait.Veclike.html) joins
together
([`Get`](https://docs.rs/maplike/latest/maplike/ops/trait.Get.html),
[`Set`](https://docs.rs/maplike/latest/maplike/ops/trait.Set.html),
[`Push`](https://docs.rs/maplike/latest/maplike/ops/trait.Push.html),
[`Pop`](https://docs.rs/maplike/latest/maplike/ops/trait.Pop.html),
[`Clear`](https://docs.rs/maplike/latest/maplike/ops/trait.Clear.html),
[`Len`](https://docs.rs/maplike/latest/maplike/ops/trait.Len.html), and
[`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)), thus allowing
for code that is generic over
[`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html),
[`VecDeque`](https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html),
`smallvec::SmallVec`,
[`tinyvec::ArrayVec`](https://docs.rs/tinyvec/latest/tinyvec/struct.ArrayVec.html), and
[`tinyvec::TinyVec`](https://docs.rs/tinyvec/latest/tinyvec/enum.TinyVec.html).

```rust
use maplike::abc::{Keyed, Veclike};
use maplike::ops::{Clear, Push};

// This function is generic over any `Veclike` collection. The `Veclike` bound
// provides `.clear()`, `.push()` and many other methods at once.
fn replace_all<C: Veclike<usize, Value = i32>>(collection: &mut C, values: &[i32]) {
    collection.clear();
    for &value in values {
        collection.push(value);
    }
}

// `replace_all()` now works for any `Veclike` collection.

// Works on `Vec`,
let mut vec = Vec::new();
replace_all(&mut vec, &[1, 2, 3]);
assert_eq!(vec, [1, 2, 3]);
replace_all(&mut vec, &[4, 5, 6]);
assert_eq!(vec, [4, 5, 6]);

#[cfg(feature = "smallvec")]
{
    use smallvec::SmallVec;

    // Works on `smallvec::SmallVec`.
    let mut small_vec: SmallVec<[i32; 8]> = SmallVec::new();
    replace_all(&mut small_vec, &[7, 8, 9]);
    assert_eq!(small_vec.as_slice(), [7, 8, 9]);
}

#[cfg(feature = "tinyvec")]
{
    use tinyvec::{ArrayVec, TinyVec};

    // Works on `tinyvec::ArrayVec`.
    let mut tiny_array_vec: ArrayVec<[i32; 8]> = ArrayVec::new();
    replace_all(&mut tiny_array_vec, &[7, 8, 9]);
    assert_eq!(tiny_array_vec.as_slice(), [7, 8, 9]);

    // Works on `tinyvec::TinyVec`.
    let mut tiny_vec: TinyVec<[i32; 8]> = TinyVec::new();
    replace_all(&mut tiny_vec, &[10, 11, 12]);
    assert_eq!(tiny_vec.as_slice(), [10, 11, 12]);
}

// NOTE: `arrayvec::ArrayVec` and `arrayvec::ArrayString` are not `Veclike`
// because they do not implement `Index`.
```

## Traits

### Operations

This crate provides traits for common operations over map-like, set-like,
array-like, and vec-like data structures:
[`.get()`](https://docs.rs/maplike/latest/maplike/ops/trait.Get.html#tymethod.get),
[`.set()`](https://docs.rs/maplike/latest/maplike/ops/trait.Set.html#tymethod.set),
[`.modify()`](https://docs.rs/maplike/latest/maplike/ops/trait.Modify.html#tymethod.modify),
[`.insert()`](https://docs.rs/maplike/latest/maplike/ops/trait.Insert.html#tymethod.insert),
[`.remove()`](https://docs.rs/maplike/latest/maplike/ops/trait.Remove.html#tymethod.remove),
[`.swap_remove()`](https://docs.rs/maplike/latest/maplike/ops/trait.SwapRemove.html#tymethod.swap_remove),
[`.push()`](https://docs.rs/maplike/latest/maplike/ops/trait.Push.html#tymethod.push),
[`.pop()`](https://docs.rs/maplike/latest/maplike/ops/trait.Pop.html#tymethod.pop),
[`.put()`](https://docs.rs/maplike/latest/maplike/ops/trait.Put.html#tymethod.put),
[`.clear()`](https://docs.rs/maplike/latest/maplike/ops/trait.Clear.html#tymethod.clear),
[`.len()`](https://docs.rs/maplike/latest/maplike/ops/trait.Len.html#tymethod.len),
[`.resize()`](https://docs.rs/maplike/latest/maplike/ops/trait.Resize.html#tymethod.resize),
[`.with_one()`](https://docs.rs/maplike/latest/maplike/ops/trait.WithOne.html#tymethod.with_one),
[`.assign()`](https://docs.rs/maplike/latest/maplike/ops/trait.Assign.html#tymethod.assign),
[`.values()`](https://docs.rs/maplike/latest/maplike/iter/trait.Values.html#tymethod.values),
[`.into_values()`](https://docs.rs/maplike/latest/maplike/iter/trait.IntoValues.html#tymethod.into_values),
[`.iter()`](https://docs.rs/maplike/latest/maplike/iter/trait.Iter.html#tymethod.iter), and
[`.into_iter()`](https://docs.rs/maplike/latest/maplike/iter/trait.IntoIter.html#tymethod.into_iter).

For bidirectional maps, there are also variants of the get and remove operations
by left and right key:
[`.get_by_left()`](https://docs.rs/maplike/latest/maplike/ops/trait.GetByLeft.html#tymethod.get_by_left),
[`.get_by_right()`](https://docs.rs/maplike/latest/maplike/ops/trait.GetByRight.html#tymethod.get_by_right),
[`.remove_by_left()`](https://docs.rs/maplike/latest/maplike/ops/trait.RemoveByLeft.html#tymethod.remove_by_left),
[`.remove_by_right()`](https://docs.rs/maplike/latest/maplike/ops/trait.RemoveByRight.html#tymethod.remove_by_right).

### Entry API

We provide generic
[Entry](https://docs.rs/maplike/latest/maplike/entry/trait.Entry.html) API for
types that have an Entry API:
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
[`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), and
[`indexmap::IndexMap`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html).

### Containers

For brevity and convenience, we also provide
[`Container`](https://docs.rs/maplike/latest/maplike/abc/trait.Container.html),
[`Keyed`](https://docs.rs/maplike/latest/maplike/abc/trait.Keyed.html),
[`Scalarlike`](https://docs.rs/maplike/latest/maplike/abc/trait.Scalarlike.html),
[`Maplike`](https://docs.rs/maplike/latest/maplike/abc/trait.Maplike.html),
[`Setlike`](https://docs.rs/maplike/latest/maplike/abc/trait.Setlike.html),
[`Arraylike`](https://docs.rs/maplike/latest/maplike/abc/trait.Arraylike.html), and
[`Veclike`](https://docs.rs/maplike/latest/maplike/abc/trait.Veclike.html) abstract
container traits that join together traits of multiple operations.

## Supported containers

### Standard library

Rust's standard library containers are supported via built-in convenience
implementations:

- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), gated by the `std` feature (enabled by default);
- [`HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html), gated by the `std` feature (enabled by default);
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), gated by the `alloc` feature (enabled by default);
- [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html), gated by the `alloc` feature (enabled by default);
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), gated by the `alloc` feature (enabled by default);
- [`VecDeque`](https://doc.rust-lang.org/alloc/collections/vec_deque/struct.VecDeque.html), gated by the `alloc` feature (enabled by default);
- [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html), gated by the `alloc` feature (enabled by default);
- [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) and its weak pointer,
  [`std::rc::Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html), both
  gated by the `alloc` feature (enabled by default);
- [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html),
  and its weak pointer,
  [`std::sync::Weak`](https://doc.rust-lang.org/std/sync/struct.Weak.html), both
  gated by the `std` feature (enabled by default);
- [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html), not feature-gated;

### Primitives

All Rust's scalar types (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`,
`u16`, `u32`, `u64`, `u128`, `usize`, `f32`, `f64`, `char`, `bool`, `()`) are
supported and treated as single-element, `usize`-keyed maps.

Rust's compound types (arrays, tuples, slices) are supported and treated as
`usize`-keyed maps.

### `maplike`'s types

`maplike` provides and supports its own generic type,
[`One`](https://docs.rs/maplike/latest/maplike/struct.One.html), for a
collection that always has only one element. Think `Option`, but without `None`.
Or `Box`, but without pointer indirection, behaving like a collection despite
holding a value not reference, allocated on the stack.

Wrap your value in this type if you need to treat it as a single-element,
`usize`-keyed map and your type does not happen to be a Rust primitive.

### Third-party types

In addition to the standard library, `maplike` has built-in feature-gated
trait implementations for data structures from certain external crates:

- [`bidimap::BiBTreeMap`](https://docs.rs/bidimap/latest/bidimap/), gated by
  the `bidimap` feature flag, and
  [`bidimap::BiHashMap`](https://docs.rs/bidimap/latest/bidimap/), which is
  additionally gated by the `std` feature flag.
  [`bidimap`](https://github.com/urschrei/bidimap)
  is a maintained fork of the currently unmaintained
  [`bimap`](https://github.com/brson/bimap) crate;
- [`indexmap::IndexMap`](https://docs.rs/indexmap/latest/indexmap/map/struct.IndexMap.html)
  and [`indexmap::IndexSet`](https://docs.rs/indexmap/latest/indexmap/set/struct.IndexSet.html),
  gated by the `indexmap` feature flag;
- [`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/index.html), gated by the
  `rstar` feature flag;
- [`slab::Slab`](https://docs.rs/slab/latest/slab/), gated by the `slab`
  feature flag (`Insert` is not implemented; see
  [Technical sidenotes](#technical-sidenotes));
- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/),
  gated by the `stable-vec` feature flag;
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
  gated by the `thunderdome` feature flag;
- [`arrayvec::ArrayVec`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html)
  and [`arrayvec::ArrayString`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayString.html),
  gated by the `arrayvec` feature flag (individual vec-like traits, but not
  [`Veclike`](https://docs.rs/maplike/latest/maplike/ops/trait.Veclike.html), because
  `arrayvec` types do not implement
  [`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html));
- `smallvec::SmallVec`, gated by the `smallvec` feature flag;
- [`tinyvec::ArrayVec`](https://docs.rs/tinyvec/latest/tinyvec/struct.ArrayVec.html),
  and [`tinyvec::TinyVec`](https://docs.rs/tinyvec/latest/tinyvec/enum.TinyVec.html),
  gated by the `tinyvec` feature flag;
- geometry types from [`geo`](https://docs.rs/geo)/[`geo-types`](https://docs.rs/geo-types),
  gated by the `geo` feature flag:
  [`Coord`](https://docs.rs/geo/latest/geo/struct.Coord.html),
  [`Point`](https://docs.rs/geo/latest/geo/struct.Point.html),
  [`Line`](https://docs.rs/geo/latest/geo/struct.Line.html),
  [`Rect`](https://docs.rs/geo/latest/geo/struct.Rect.html),
  [`Triangle`](https://docs.rs/geo/latest/geo/struct.Triangle.html),
  [`Polygon`](https://docs.rs/geo/latest/geo/struct.Polygon.html), and
  [`Geometry`](https://docs.rs/geo/latest/geo/enum.Geometry.html)
  are treated as single-element, `usize`-keyed maps;
  [`LineString`](https://docs.rs/geo/latest/geo/struct.LineString.html),
  [`MultiPoint`](https://docs.rs/geo/latest/geo/struct.MultiPoint.html),
  [`MultiLineString`](https://docs.rs/geo/latest/geo/struct.MultiLineString.html),
  [`MultiPolygon`](https://docs.rs/geo/latest/geo/struct.MultiPolygon.html), and
  [`GeometryCollection`](https://docs.rs/geo/latest/geo/struct.GeometryCollection.html)
  are treated as `usize`-keyed vec-like maps of their elements;

For some examples of practical use, see the
[examples](https://github.com/mikwielgus/undoredo/tree/develop/examples)
directory of the [`undoredo`](https://github.com/mikwielgus/undoredo) crate.

## Unsupported containers

Among stable vector data structures,
[`SlotMap`](https://docs.rs/slotmap/latest/slotmap/) and
[`generational-arena`](https://docs.rs/generational-arena/latest/generational_arena/)
are not supported because they lack interfaces for insertion at an arbitrary
key.

### Technical sidenotes

Unlike maps and sets, not all stable vector data
structures allow insertion and removal at arbitrary indexes regardless of
whether they are vacant, occupied or out of bounds. For `StableVec`, we managed
to implement inserting at out-of-bound indexes by changing the length before
insertion using the
[`.reserve_for()`](https://docs.rs/stable-vec/latest/stable_vec/struct.StableVecFacade.html#method.reserve_for)
method. For `thunderdome::Arena`, we insert at arbitrary key directly via the
[`.insert_at()`](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html#method.insert_at)
method.

For `Slab`, an interface to insert at an arbitrary key is missing apparently
[because](https://github.com/tokio-rs/slab/issues/117#issuecomment-1159741097)
the [freelist](https://en.wikipedia.org/wiki/Free_list) `Slab` uses to keep
track of its vacant indexes is only singly-linked, not doubly-linked. Inserting
an element at an arbitrary vacant index would require removing that index from
the freelist. But since there is no backwards link available at a given key,
doing so would require traversing the freelist from the beginning to find the
position of the previous node, which would incur a slow `O(n)` time cost.

Because of that, `Slab` implements the most of the traits, but not
[`Insert`](https://docs.rs/maplike/latest/maplike/ops/trait.Insert.html)
(and therefore it is also not
[`Maplike`](https://docs.rs/maplike/latest/maplike/abc/trait.Maplike.html)).
