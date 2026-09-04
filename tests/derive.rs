// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "derive")]

use maplike::ops::Assign;

#[derive(maplike::Keyed, Debug, PartialEq)]
struct TestStruct {
    integer: i64,
    string: String,
}

#[derive(maplike::Keyed, Debug, PartialEq)]
enum TestEnum {
    Usize(usize),
    Strings(String, String),
}

fn assert_derived_keyed<T>()
where
    T: maplike::abc::Keyed<Key = usize, Value = T>,
{
}

#[test]
fn test_assign_on_struct() {
    assert_derived_keyed::<TestStruct>();

    let mut s = TestStruct {
        integer: 0,
        string: "old".to_string(),
    };

    s.assign(TestStruct {
        integer: 1,
        string: "new".to_string(),
    });

    assert_eq!(
        s,
        TestStruct {
            integer: 1,
            string: "new".to_string(),
        }
    );
}

#[test]
fn test_assign_on_enum() {
    assert_derived_keyed::<TestEnum>();

    let mut e = TestEnum::Usize(0);

    e.assign(TestEnum::Strings(
        "string1".to_string(),
        "string2".to_string(),
    ));

    assert_eq!(
        e,
        TestEnum::Strings("string1".to_string(), "string2".to_string())
    );
}
