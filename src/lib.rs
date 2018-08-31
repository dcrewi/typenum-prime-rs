// src/lib.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Compile-time primality testing of `typenum` integers.
//!
//! The current algorithm is trial division by every prime number from
//! `2` up to `next_integer_power_of_two(sqrt(n))`. The default
//! compiler recursion limit could sometimes be insufficient,
//! depending on the magnitude of the integer being tested. When
//! necessary, raise it using a crate attribute:
//!
//! ```
//! #![recursion_limit="128"]
//! ```
//!
//! ## Example
//!
//! The intended use of this crate is to put a bound on type-level
//! integer parameters so the compiler enforces their primality. For
//! instance, you might want the number of buckets in a
//! statically-sized hash table to always be a prime number so that
//! hash collisions are reduced. Now you can let the compiler enforce
//! this constraint.
//!
//! ```ignore
//! pub struct StaticHashTable<K,V,N>
//!     where N: Prime + ArrayLength<Option<(K,V)>> {
//!     buckets: GenericArray<Option<(K,V)>,N>
//! }
//! ```

#![no_std]
#![warn(missing_docs)]

#[cfg_attr(test, macro_use)] pub extern crate typenum;

use typenum::marker_traits::{Bit, Unsigned};
use typenum::consts::True;

use private::PrivateIsPrime;


#[doc(hidden)]
pub mod private;


// Test all integers from 0 through 1024, inclusive.
#[cfg(test)] pub mod test_small_constants;


/// **Type operator** for primality testing.
///
/// This trait is implemented for all unsignd integers from the
/// `typenum` crate.
pub trait IsPrime: Unsigned {
    /// A boolean indicating the result of the primality test.
    type Output: Bit;
}

impl<N> IsPrime for N where N: Unsigned + PrivateIsPrime {
    type Output = <N as PrivateIsPrime>::Output;
}


/// **Marker trait** for prime, unsigned integers; equivalent to `IsPrime<Output=True>`
///
/// This trait is automatically implemented for unsigned integers from
/// the `typenum` crate that are prime. It is not defined for 0, 1,
/// and composite integers.
pub trait Prime: Unsigned {}

impl<N> Prime for N where N: Unsigned + IsPrime<Output=True> {}
