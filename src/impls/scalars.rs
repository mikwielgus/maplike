// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::abc::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Get, Len, Modify, Put, Set, WithOne};

macro_rules! impl_traits_for_scalar {
    ($($t:ty),*) => {
        $(
            impl Container for $t {
                type Key = usize;
                type Value = Self;
            }

            impl WithOne<$t> for $t {
                #[inline(always)]
                fn with_one(value: Self) -> Self {
                    value
                }
            }

            impl Get<usize> for $t {
                #[inline(always)]
                fn get(&self, index: &usize) -> Option<&Self> {
                    if *index == 0 { Some(&self) } else { None }
                }
            }

            impl Set<usize> for $t {
                type Output = Option<Self>;

                #[inline(always)]
                fn set(&mut self, index: usize, value: Self) -> Option<Self> {
                    assert_eq!(index, 0);
                    Some(core::mem::replace(self, value))
                }
            }

            impl Modify<usize> for $t {
                #[inline(always)]
                fn modify<F>(&mut self, index: &usize, f: F) where F: FnOnce(&mut Self) {
                    assert_eq!(*index, 0);
                    f(self)
                }
            }

            impl Put<$t> for $t {
                #[inline(always)]
                fn put(&mut self, value: Self) -> Option<Self> {
                    Some(core::mem::replace(self, value))
                }
            }

            impl Len for $t {
                #[inline(always)]
                fn len(&self) -> usize {
                    1
                }
            }

            impl<'a> Values<'a> for $t {
                type Values = core::iter::Once<&'a Self>;

                #[inline(always)]
                fn values(&'a self) -> Self::Values {
                    core::iter::once(self)
                }
            }

            impl IntoValues for $t {
                type IntoValues = core::iter::Once<Self>;

                #[inline(always)]
                fn into_values(self) -> Self::IntoValues {
                    core::iter::once(self)
                }
            }

            impl<'a> Iter<'a, usize> for $t {
                type Iter = core::iter::Once<(usize, &'a Self)>;

                #[inline(always)]
                fn iter(&'a self) -> Self::Iter {
                    core::iter::once((0, self))
                }
            }

            impl IntoIter<usize> for $t {
                type IntoIter = core::iter::Enumerate<core::iter::Once<Self>>;

                #[inline(always)]
                fn into_iter(self) -> Self::IntoIter {
                    core::iter::once(self).enumerate()
                }
            }
        )*
    };
}

impl_traits_for_scalar!(i8, i16, i32, i64, i128, isize);
impl_traits_for_scalar!(u8, u16, u32, u64, u128, usize);
impl_traits_for_scalar!(f32, f64);
impl_traits_for_scalar!(char, bool, ());
