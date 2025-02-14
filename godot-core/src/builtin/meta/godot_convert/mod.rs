/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub(crate) mod convert_error;
mod impls;

pub use convert_error::ConvertError;

use crate::builtin::Variant;

use super::{GodotFfiVariant, GodotType};

/// Indicates that a type can be passed to/from Godot, either directly or through an intermediate "via" type.
///
/// The associated type `Via` specifies _how_ this type is passed across the FFI boundary to/from Godot.
/// Generally [`ToGodot`] needs to be implemented to pass a type to Godot, and [`FromGodot`] to receive this type from Godot.
///
/// [`GodotType`] is a stronger bound than [`GodotConvert`], since it expresses that a type is _directly_ representable
/// in Godot (without intermediate "via"). Every `GodotType` also implements `GodotConvert` with `Via = Self`.
pub trait GodotConvert {
    /// The type through which `Self` is represented in Godot.
    type Via: GodotType;
}

/// Defines the canonical conversion to Godot for a type.
///
/// It is assumed that all the methods return equal values given equal inputs. Additionally it is assumed
/// that if [`FromGodot`] is implemented, converting to Godot and back again will return a value equal to the
/// starting value.
///
/// Violating these assumptions is safe but will give unexpected results.
pub trait ToGodot: Sized + GodotConvert {
    /// Converts this type to the Godot type by reference, usually by cloning.
    fn to_godot(&self) -> Self::Via;

    /// Converts this type to the Godot type.
    ///
    /// This can in some cases enable minor optimizations, such as avoiding reference counting operations.
    fn into_godot(self) -> Self::Via {
        self.to_godot()
    }

    /// Converts this type to a [Variant].
    fn to_variant(&self) -> Variant {
        self.to_godot().to_ffi().ffi_to_variant()
    }
}

/// Defines the canonical conversion from Godot for a type.
///
/// It is assumed that all the methods return equal values given equal inputs. Additionally it is assumed
/// that if [`ToGodot`] is implemented, converting to Godot and back again will return a value equal to the
/// starting value.
///
/// Violating these assumptions is safe but will give unexpected results.
pub trait FromGodot: Sized + GodotConvert {
    /// Performs the conversion.
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError>;

    /// ⚠️ Performs the conversion.
    ///
    /// # Panics
    /// If the conversion fails.
    fn from_godot(via: Self::Via) -> Self {
        Self::try_from_godot(via).unwrap()
    }

    /// Performs the conversion from a [`Variant`].
    fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError> {
        let ffi = <Self::Via as GodotType>::Ffi::ffi_from_variant(variant)?;

        let via = Self::Via::try_from_ffi(ffi)?;
        Self::try_from_godot(via)
    }

    /// ⚠️ Performs the conversion from a [`Variant`].
    ///
    /// # Panics
    /// If the conversion fails.
    fn from_variant(variant: &Variant) -> Self {
        Self::try_from_variant(variant).unwrap()
    }
}

pub(crate) fn into_ffi<T: ToGodot>(value: T) -> <T::Via as GodotType>::Ffi {
    value.into_godot().into_ffi()
}

pub(crate) fn try_from_ffi<T: FromGodot>(
    ffi: <T::Via as GodotType>::Ffi,
) -> Result<T, ConvertError> {
    let via = <T::Via as GodotType>::try_from_ffi(ffi)?;
    T::try_from_godot(via)
}

macro_rules! impl_godot_as_self {
    ($T:ty) => {
        impl $crate::builtin::meta::GodotConvert for $T {
            type Via = $T;
        }

        impl $crate::builtin::meta::ToGodot for $T {
            #[inline]
            fn to_godot(&self) -> Self::Via {
                self.clone()
            }

            #[inline]
            fn into_godot(self) -> Self::Via {
                self
            }
        }

        impl $crate::builtin::meta::FromGodot for $T {
            #[inline]
            fn try_from_godot(via: Self::Via) -> Result<Self, $crate::builtin::meta::ConvertError> {
                Ok(via)
            }
        }
    };
}

pub(crate) use impl_godot_as_self;
