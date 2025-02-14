/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! Godot-types that are Strings.

mod gstring;
mod macros;
mod node_path;
mod string_chars;
mod string_name;

pub use gstring::*;
pub use node_path::*;
pub use string_name::*;

use super::meta::{ConvertError, FromGodot, GodotConvert, ToGodot};

impl GodotConvert for &str {
    type Via = GString;
}

impl ToGodot for &str {
    fn to_godot(&self) -> Self::Via {
        GString::from(*self)
    }
}

impl GodotConvert for String {
    type Via = GString;
}

impl ToGodot for String {
    fn to_godot(&self) -> Self::Via {
        GString::from(self)
    }
}

impl FromGodot for String {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        Ok(via.to_string())
    }
}
