<!--
SPDX-FileCopyrightText: 2026 maplike contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Docs](https://docs.rs/maplike/badge.svg)](https://docs.rs/maplike/)
[![Crates.io](https://img.shields.io/crates/v/maplike.svg)](https://crates.io/crates/maplike)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/maplike.svg)](#licence)

# maplike

Traits for abstract containers and operations over them.

This crate provides traits for common operations over map-like, set-like, and
vec-like data structures:
[`get`](https://docs.rs/maplike/latest/maplike/trait.Get.html#tymethod.get),
[`set`](https://docs.rs/maplike/latest/maplike/trait.Set.html#tymethod.set),
[`modify`](https://docs.rs/maplike/latest/maplike/trait.Modify.html#tymethod.modify),
[`insert`](https://docs.rs/maplike/latest/maplike/trait.Insert.html#tymethod.insert),
[`remove`](https://docs.rs/maplike/latest/maplike/trait.Remove.html#tymethod.remove),
[`push`](https://docs.rs/maplike/latest/maplike/trait.Push.html#tymethod.push),
[`pop`](https://docs.rs/maplike/latest/maplike/trait.Pop.html#tymethod.pop),
[`clear`](https://docs.rs/maplike/latest/maplike/trait.Clear.html#tymethod.clear),
[`len`](https://docs.rs/maplike/latest/maplike/trait.Len.html#tymethod.len),
[`assign`](https://docs.rs/maplike/latest/maplike/trait.Assign.html#tymethod.assign), and
[`into_iter`](https://docs.rs/maplike/latest/maplike/trait.IntoIter.html#tymethod.into_iter).

For bidirectional maps, there are also left-key and right-key variants of
the get and remove operations:
[`get_by_left`](https://docs.rs/maplike/latest/maplike/trait.GetByLeft.html#tymethod.get_by_left),
[`get_by_right`](https://docs.rs/maplike/latest/maplike/trait.GetByRight.html#tymethod.get_by_right),
[`remove_by_left`](https://docs.rs/maplike/latest/maplike/trait.RemoveByLeft.html#tymethod.remove_by_left),
[`remove_by_right`](https://docs.rs/maplike/latest/maplike/trait.RemoveByRight.html#tymethod.remove_by_right).

For brevity and convenience, we also provide
[`Scalarlike`](https://docs.rs/maplike/latest/maplike/trait.Scalarlike.html),
[`Maplike`](https://docs.rs/maplike/latest/maplike/trait.Maplike.html),
[`Setlike`](https://docs.rs/maplike/latest/maplike/trait.Setlike.html),
[`Arraylike`](https://docs.rs/maplike/latest/maplike/trait.Arraylike.html), and
[`Veclike`](https://docs.rs/maplike/latest/maplike/trait.Veclike.html) traits,
which represent complete abstract containers that join together traits of
multiple operations.

## Usage

### Adding dependency

First, add `maplike` as a dependency to your Cargo.toml:

```toml
[dependencies]
maplike = { version = "0.11.1", features = ["derive"] }
```

The `derive` feature flag is only needed if you want to derive `Assign`
or `Container` traits using derive macros: `#[derive(Assign)]` or
`#[derive(Container)]`.

### Usage examples

`maplike`'s traits allow you to write functions that are generic over many
different collection types. A single trait like
[`Get`](https://docs.rs/maplike/latest/maplike/trait.Get.html) is enough to
abstract over vectors, arrays, and maps alike.

```rust
use std::collections::BTreeMap;

use maplike::Get;

// Generic over any collection implementing the `Get` trait.
fn get_second_element<C: Get<usize>>(collection: &C) -> Option<&C::Value> {
    collection.get(&1)
}

// `get_second_element()` works for vectors, arrays, and maps with the very
// same code.
assert_eq!(get_second_element(&vec![10, 20, 30]), Some(&20));
assert_eq!(get_second_element(&[10, 20, 30]), Some(&20));
assert_eq!(get_second_element(&BTreeMap::from([(0usize, 10), (1usize, 20)])), Some(&20));
```

An abstract container trait like
[`Veclike`](https://docs.rs/maplike/latest/maplike/trait.Veclike.html) bundles
several traits
([`Get`](https://docs.rs/maplike/latest/maplike/trait.Get.html),
[`Set`](https://docs.rs/maplike/latest/maplike/trait.Set.html),
[`Push`](https://docs.rs/maplike/latest/maplike/trait.Push.html),
[`Pop`](https://docs.rs/maplike/latest/maplike/trait.Pop.html),
[`Clear`](https://docs.rs/maplike/latest/maplike/trait.Clear.html),
[`Len`](https://docs.rs/maplike/latest/maplike/trait.Len.html), and
[`Index`](https://doc.rust-lang.org/std/ops/trait.Index.html)) together behind
one bound:

```rust
use maplike::{Clear, Push, Veclike};

// This function is generic over any `Veclike` collection. The `Veclike` bound
// provides `clear`, `push`, and many other methods at once.
fn replace_all<C: Veclike<usize, Value = i32>>(collection: &mut C, values: &[i32]) {
    collection.clear();
    for &value in values {
        collection.push(value);
    }
}

// `replace_all` works for any `Veclike` collection, such as `Vec`.
let mut vec = Vec::new();
replace_all(&mut vec, &[1, 2, 3]);
assert_eq!(vec, [1, 2, 3]);
replace_all(&mut vec, &[4, 5, 6]);
assert_eq!(vec, [4, 5, 6]);
```

## Supported collections

### Standard library

Rust's standard library collections are supported via built-in convenience
implementations:

- [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), gated by the `std` feature (enabled by default);
- [`HashSet`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html), gated by the `std` feature (enabled by default);
- [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html), not feature-gated;
- [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html), not feature-gated;
- [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), not feature-gated, but does not support stable removal.

### Third-party types

In addition to the standard library, `maplike` has built-in feature-gated
convenience implementations for data structures from certain external crates:

- [`bidimap::BiBTreeMap`](https://docs.rs/bidimap/latest/bidimap/), gated by
  the `bidimap` feature flag, and
  [`bidimap::BiHashMap`](https://docs.rs/bidimap/latest/bidimap/), which is also
  gated by the `std` feature flag.
  [`bidimap`](https://github.com/urschrei/bidimap)
  is a maintained fork of the currently unmaintained
  [`bimap`](https://github.com/brson/bimap) crate;
- [`rstar::RTree`](https://docs.rs/rstar/0.12.2/rstar/index.html), gated by the
  `rstar` feature flag;
- [`rstared::RTreed`](https://docs.rs/rstared/latest/rstared/), gated by the
  `rstared` feature flag;
- [`stable_vec::StableVec`](https://docs.rs/stable-vec/latest/stable_vec/),
  gated by the `stable-vec` feature flag;
- [`thunderdome::Arena`](https://docs.rs/thunderdome/latest/thunderdome/),
  gated by the `thunderdome` feature flag.

For examples, see
[examples](https://github.com/mikwielgus/undoredo/tree/develop/examples)
directory of the [`undoredo`](https://github.com/mikwielgus/undoredo) crate.

## Unsupported collections

Standard library's `VecDeque` is unsupported.

Among stable vector data structures,
[`Slab`](https://docs.rs/slab/latest/slab/),
[`SlotMap`](https://docs.rs/slotmap/latest/slotmap/),
[`generational-arena`](https://docs.rs/generational-arena/latest/generational_arena/)
cannot be supported because they lack interfaces for insertion at an arbitrary
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
method. Collections for which we could not achieve this are documented in the
section below.

For `Slab`, an interface to insert at an arbitrary key is missing apparently
[because](https://github.com/tokio-rs/slab/issues/117#issuecomment-1159741097)
the [freelist](https://en.wikipedia.org/wiki/Free_list) `Slab` uses to keep
track of its vacant indexes is only singly-linked, not doubly-linked. Inserting
an element at an arbitrary vacant index would require removing that index from
the freelist. But since there is no backwards link available at a given key,
doing so would require traversing the freelist from the beginning to find the
position of the previous node, which would incur an overly slow `O(n)` time
cost.
