// SPDX-FileCopyrightText: 2025 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/maplike")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, doc = "\n## Feature flags\n")]
#![cfg_attr(docsrs, doc = document_features::document_features!())]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std as std_;

#[cfg(feature = "alloc")]
extern crate alloc as alloc_;

pub mod containers;
pub mod entry;
pub mod iter;
pub mod one;
pub mod ops;

#[cfg(feature = "derive")]
pub use maplike_derive::Container;

mod impls;
