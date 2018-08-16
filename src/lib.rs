// src/lib.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Compile-time primality testing for `typenum` integers.
//!
//! The current algorithm is trial division by every number between 2
//! and `floor(sqrt(n))` inclusive. This deepens the compiler
//! recursion level once for every division test required plus
//! additional recursion for the division. The default compiler
//! recursion limit may be insufficient, depending on the integer
//! being tested. Raise it using a crate attribute:
//!
//! ```
//! #![recursion_limit="128"]
//! ```
//!
//! ## Example
//!
//! The intended use of the crate is to bound generic `typenum`
//! parameters so they are always prime. For instance, you might want
//! a statically-sized hash table to always have a prime number of
//! buckets to reduce collisions of a low-budget hash algorithm.
//!
//! ```ignore
//! pub struct StaticHashTable<K,V,N>
//!     where N: Prime + ArrayLength<Option<(K,V)>> {
//!     buckets: GenericArray<Option<(K,V)>,N>
//! }
//! ```

#![no_std]
#![warn(missing_docs)]
// FIXME it would be nice if so much recursion was not necessary
#![cfg_attr(test, recursion_limit="128")]

extern crate typenum;

use typenum::marker_traits::{Bit, Unsigned};
use typenum::consts::True;

use private::PrivateIsPrime;


#[doc(hidden)]
pub mod private;


/// **Type operator** for primality testing.
///
/// This trait should not be implemented outside this crate.
pub trait IsPrime: Unsigned {
    /// A boolean indicating the result of the primality test.
    type Output: Bit;
}

impl<N> IsPrime for N where N: PrivateIsPrime {
    type Output = <N as PrivateIsPrime>::Output;
}

// Test all integers from 0 through 1024, inclusive.
include!(concat!(env!("OUT_DIR"), "/build-script-generated-tests.rs"));


/// **Marker trait** for prime, unsigned integers.
///
/// This trait should not be implemented outside this crate.
///
/// This trait is automatically implemented for typenum unsigned
/// integers that are prime. It is not defined for 0, 1, and composite
/// integers.
///
/// Bounding by this trait is equivalent to bounding by
/// `IsPrime<Output=True>`, which is how it's implemented.
pub trait Prime: Unsigned {}

impl<N> Prime for N where N: Unsigned + IsPrime<Output=True> {}
