//! Facilities for creating and reading objects on the memory of an
//! AssemblyScript (Asc) WASM module. Objects are passed through the `asc_new`
//! and `asc_get` methods of an `AscHeap` implementation. These methods take
//! types that implement `To`/`FromAscObj` and are therefore convertible to/from
//! an `AscType`. Implementations of `AscType` live in the `class` module.
//! Implementations of `To`/`FromAscObj` live in the `to_from` module.

pub use self::asc_ptr::AscPtr;
use crate::error::DeterministicHostError;
use graph::prelude::anyhow;
use std::convert::TryInto;
use std::mem::size_of;

pub mod asc_ptr;
pub mod class;

// WASM is little-endian, and for simplicity we currently assume that the host
// is also little-endian.
#[cfg(target_endian = "big")]
compile_error!("big-endian targets are currently unsupported");

/// An Asc primitive or an `AscPtr` into the Asc heap. A type marked as
/// `AscValue` must have the same byte representation in Rust and Asc, including
/// same size, and size must be equal to alignment.
// `AscValue` isn't really public.
pub trait AscValue: AscType + Copy + Default {}

impl AscType for bool {
    fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
        Ok(vec![*self as u8])
    }

    fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError> {
        if asc_obj.len() != 1 {
            Err(DeterministicHostError(anyhow::anyhow!(
                "Incorrect size for bool. Expected 1, got {},",
                asc_obj.len()
            )))
        } else {
            Ok(asc_obj[0] != 0)
        }
    }
}

impl AscValue for bool {}
impl<T> AscValue for AscPtr<T> {}

macro_rules! impl_asc_type {
    ($($T:ty),*) => {
        $(
            impl AscType for $T {
                fn to_asc_bytes(&self) -> Result<Vec<u8>, DeterministicHostError> {
                    Ok(self.to_le_bytes().to_vec())
                }

                fn from_asc_bytes(asc_obj: &[u8]) -> Result<Self, DeterministicHostError> {
                    let bytes = asc_obj.try_into().map_err(|_| {
                        DeterministicHostError(anyhow::anyhow!(
                            "Incorrect size for {}. Expected {}, got {},",
                            stringify!($T),
                            size_of::<Self>(),
                            asc_obj.len()
                        ))
                    })?;

                    Ok(Self::from_le_bytes(bytes))
                }
            }

            impl AscValue for $T {}
        )*
    };
}

impl_asc_type!(u8, u16, u32, u64, i8, i32, i64, f32, f64);
