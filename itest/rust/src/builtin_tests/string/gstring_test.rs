/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;

use crate::framework::itest;
use godot::builtin::GString;

// TODO use tests from godot-rust/gdnative

#[itest]
fn string_default() {
    let string = GString::new();
    let back = String::from(&string);

    assert_eq!(back.as_str(), "");
}

#[itest]
fn string_conversion() {
    let string = String::from("some string");
    let second = GString::from(&string);
    let back = String::from(&second);

    assert_eq!(string, back);

    let second = GString::from(string.clone());
    let back = String::from(second);

    assert_eq!(string, back);
}

#[itest]
fn string_equality() {
    let string = GString::from("some string");
    let second = GString::from("some string");
    let different = GString::from("some");

    assert_eq!(string, second);
    assert_ne!(string, different);
}

#[itest]
fn string_ordering() {
    let low = GString::from("Alpha");
    let high = GString::from("Beta");

    assert!(low < high);
    assert!(low <= high);
    assert!(high > low);
    assert!(high >= low);
}

#[itest]
fn string_clone() {
    let first = GString::from("some string");
    #[allow(clippy::redundant_clone)]
    let cloned = first.clone();

    assert_eq!(first, cloned);
}

#[itest]
fn empty_string_chars() {
    // Tests regression from #228: Null pointer passed to slice::from_raw_parts
    let s = GString::new();
    assert_eq!(s.chars_checked(), &[]);
    assert_eq!(unsafe { s.chars_unchecked() }, &[]);
}

#[itest]
fn string_chars() {
    let string = String::from("some_string");
    let string_chars: Vec<char> = string.chars().collect();
    let godot_string = GString::from(string);
    let godot_string_chars: Vec<char> = godot_string.chars_checked().to_vec();

    assert_eq!(godot_string_chars, string_chars);
}

#[itest]
fn string_hash() {
    let set: HashSet<GString> = [
        "string_1",
        "SECOND string! :D",
        "emoji time: 😎",
        r#"got/!()%)=!"/]}¡[$½{¥¡}@£symbol characters"#,
        "some garbageTƉ馧쟻�韂󥢛ꮛ૎ཾ̶D@/8ݚ򹾴-䌗򤷨񄣷8",
    ]
    .into_iter()
    .map(GString::from)
    .collect();
    assert_eq!(set.len(), 5);
}
