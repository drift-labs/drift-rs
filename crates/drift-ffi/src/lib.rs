//! Drift program FFI exports

// c-abi compat for 128-bit integers is required (i.e rust >= 1.77.0)
static_assertions::const_assert_eq!(align_of::<i128>(), 16);
static_assertions::const_assert_eq!(align_of::<u128>(), 16);

pub mod types;

#[cfg(feature = "exports")]
mod exports;
