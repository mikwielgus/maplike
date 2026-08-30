// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use geo_types::{
    Coord, CoordNum, Geometry, GeometryCollection, Line, LineString, MultiLineString, MultiPoint,
    MultiPolygon, Point, Polygon, Rect, Triangle,
};

use crate::containers::Container;
use crate::iter::{IntoIter, IntoValues, Iter, Values};
use crate::ops::{Clear, Get, Len, Modify, Pop, Push, Put, Resize, Set, WithOne};

macro_rules! impl_traits_for_geo_noncollection {
    ($($ty:ident),* $(,)?) => {
        $(
            impl<T: CoordNum> Container for $ty<T> {
                type Key = usize;
                type Value = Self;
            }

            impl<T: CoordNum> WithOne<$ty<T>> for $ty<T> {
                #[inline(always)]
                fn with_one(value: Self) -> Self {
                    value
                }
            }

            impl<T: CoordNum> Get<usize> for $ty<T> {
                #[inline(always)]
                fn get(&self, index: &usize) -> Option<&Self> {
                    if *index == 0 {
                        Some(self)
                    } else {
                        None
                    }
                }
            }

            impl<T: CoordNum> Set<usize> for $ty<T> {
                type Output = Option<Self>;

                #[inline(always)]
                fn set(&mut self, index: usize, value: Self) -> Option<Self> {
                    assert_eq!(index, 0);
                    Some(core::mem::replace(self, value))
                }
            }

            impl<T: CoordNum> Modify<usize> for $ty<T> {
                #[inline(always)]
                fn modify<F>(&mut self, index: &usize, f: F)
                where
                    F: FnOnce(&mut Self),
                {
                    assert_eq!(*index, 0);
                    f(self)
                }
            }

            impl<T: CoordNum> Put<$ty<T>> for $ty<T> {
                #[inline(always)]
                fn put(&mut self, value: Self) -> Option<Self> {
                    Some(core::mem::replace(self, value))
                }
            }

            impl<T: CoordNum> Len for $ty<T> {
                #[inline(always)]
                fn len(&self) -> usize {
                    1
                }
            }

            impl<'a, T: CoordNum + 'a> Values<'a> for $ty<T> {
                type Values = core::iter::Once<&'a Self>;

                #[inline(always)]
                fn values(&'a self) -> Self::Values {
                    core::iter::once(self)
                }
            }

            impl<T: CoordNum> IntoValues for $ty<T> {
                type IntoValues = core::iter::Once<Self>;

                #[inline(always)]
                fn into_values(self) -> Self::IntoValues {
                    core::iter::once(self)
                }
            }

            impl<'a, T: CoordNum + 'a> Iter<'a, usize> for $ty<T> {
                type Iter = core::iter::Once<(usize, &'a Self)>;

                #[inline(always)]
                fn iter(&'a self) -> Self::Iter {
                    core::iter::once((0, self))
                }
            }

            impl<T: CoordNum> IntoIter<usize> for $ty<T> {
                type IntoIter = core::iter::Enumerate<core::iter::Once<Self>>;

                #[inline(always)]
                fn into_iter(self) -> Self::IntoIter {
                    core::iter::once(self).enumerate()
                }
            }
        )*
    };
}

impl_traits_for_geo_noncollection!(Coord, Point, Line, Rect, Triangle, Polygon, Geometry);

macro_rules! impl_traits_for_geo_veclike {
    ($wrapper:ident, $value:ty, $ctor:ident) => {
        impl<T: CoordNum> Container for $wrapper<T> {
            type Key = usize;
            type Value = $value;
        }

        impl<T: CoordNum> WithOne<$value> for $wrapper<T> {
            #[inline(always)]
            fn with_one(element: $value) -> Self {
                Self::$ctor(alloc_::vec![element])
            }
        }

        impl<T: CoordNum> Get<usize> for $wrapper<T> {
            #[inline(always)]
            fn get(&self, index: &usize) -> Option<&$value> {
                self.0.as_slice().get(*index)
            }
        }

        impl<T: CoordNum> Set<usize> for $wrapper<T> {
            type Output = ();

            #[inline(always)]
            fn set(&mut self, index: usize, value: $value) {
                self.0.as_mut_slice()[index] = value;
            }
        }

        impl<T: CoordNum> Modify<usize> for $wrapper<T> {
            #[inline(always)]
            fn modify<F>(&mut self, index: &usize, f: F)
            where
                F: FnOnce(&mut $value),
            {
                f(&mut self.0.as_mut_slice()[*index]);
            }
        }

        impl<T: CoordNum> Push<usize> for $wrapper<T> {
            #[inline(always)]
            fn push(&mut self, value: $value) -> usize {
                alloc_::vec::Vec::push(&mut self.0, value);

                alloc_::vec::Vec::len(&self.0) - 1
            }
        }

        impl<T: CoordNum> Pop for $wrapper<T> {
            #[inline(always)]
            fn pop(&mut self) -> Option<$value> {
                alloc_::vec::Vec::pop(&mut self.0)
            }
        }

        impl<T: CoordNum> Put<$value> for $wrapper<T> {
            #[inline(always)]
            fn put(&mut self, value: $value) -> Option<$value> {
                alloc_::vec::Vec::push(&mut self.0, value);

                None
            }
        }

        impl<T: CoordNum> Clear for $wrapper<T> {
            #[inline(always)]
            fn clear(&mut self) {
                alloc_::vec::Vec::clear(&mut self.0);
            }
        }

        impl<T: CoordNum> Len for $wrapper<T> {
            #[inline(always)]
            fn len(&self) -> usize {
                alloc_::vec::Vec::len(&self.0)
            }
        }

        impl<T: CoordNum> Resize for $wrapper<T> {
            #[inline(always)]
            fn resize(&mut self, new_len: usize, value: $value)
            where
                $value: Clone,
            {
                alloc_::vec::Vec::resize(&mut self.0, new_len, value);
            }
        }

        impl<'a, T: CoordNum + 'a> Values<'a> for $wrapper<T> {
            type Values = core::slice::Iter<'a, $value>;

            #[inline(always)]
            fn values(&'a self) -> Self::Values {
                self.0.as_slice().iter()
            }
        }

        impl<T: CoordNum> IntoValues for $wrapper<T> {
            type IntoValues = alloc_::vec::IntoIter<$value>;

            #[inline(always)]
            fn into_values(self) -> Self::IntoValues {
                IntoIterator::into_iter(self.0)
            }
        }

        impl<'a, T: CoordNum + 'a> Iter<'a, usize> for $wrapper<T> {
            type Iter = core::iter::Enumerate<core::slice::Iter<'a, $value>>;

            #[inline(always)]
            fn iter(&'a self) -> Self::Iter {
                self.0.as_slice().iter().enumerate()
            }
        }

        impl<T: CoordNum> IntoIter<usize> for $wrapper<T> {
            type IntoIter = core::iter::Enumerate<alloc_::vec::IntoIter<$value>>;

            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter {
                IntoIterator::into_iter(self.0).enumerate()
            }
        }
    };
}

impl_traits_for_geo_veclike!(LineString, Coord<T>, new);
impl_traits_for_geo_veclike!(MultiPoint, Point<T>, new);
impl_traits_for_geo_veclike!(MultiLineString, LineString<T>, new);
impl_traits_for_geo_veclike!(MultiPolygon, Polygon<T>, new);
impl_traits_for_geo_veclike!(GeometryCollection, Geometry<T>, new_from);
