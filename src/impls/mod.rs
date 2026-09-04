// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod compounds;
mod option;
mod scalars;

#[cfg(feature = "std")]
mod std;

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "bidimap")]
mod bibtreemap;

#[cfg(all(feature = "bidimap", feature = "std"))]
mod bihashmap;

#[cfg(feature = "indexmap")]
mod indexmap;

#[cfg(feature = "indexmap")]
mod indexset;

#[cfg(feature = "rstar")]
mod rstar;

#[cfg(feature = "stable-vec")]
mod stable_vec;

#[cfg(feature = "thunderdome")]
mod thunderdome;

#[cfg(feature = "arrayvec")]
mod arrayvec;

#[cfg(feature = "arrayvec")]
mod arraystring;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(feature = "tinyvec")]
mod tinyvec;

#[cfg(feature = "geo")]
mod geo;
