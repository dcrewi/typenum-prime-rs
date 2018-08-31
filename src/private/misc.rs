// src/private/misc.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use core::ops::Add;

use typenum::bit::{Bit, B0};
use typenum::consts::{False, True};
use typenum::operator_aliases::Sum;
use typenum::uint::{Unsigned, UInt, UTerm};


// A type operator for dividing Self by 2, rounding up if Self is odd.
pub trait CeilDivBy2 {
    type Output;
}

impl CeilDivBy2 for UTerm {
    type Output = UTerm;
}

impl<U,B> CeilDivBy2 for UInt<U,B>
    where U: Unsigned,
          B: Bit,
          U: Add<B> {
    type Output = Sum<U,B>;
}

// Alias for the result of CeilDivBy2.
pub type CeilDivBy2Out<N> = <N as CeilDivBy2>::Output;

#[test]
fn test_ceil_div_by_2() {
    use typenum::consts::*;
    assert_type_eq!(CeilDivBy2Out<U0>, U0);
    assert_type_eq!(CeilDivBy2Out<U1>, U1);
    assert_type_eq!(CeilDivBy2Out<U2>, U1);
    assert_type_eq!(CeilDivBy2Out<U3>, U2);
    assert_type_eq!(CeilDivBy2Out<U4>, U2);
    assert_type_eq!(CeilDivBy2Out<U5>, U3);
    assert_type_eq!(CeilDivBy2Out<U6>, U3);
    assert_type_eq!(CeilDivBy2Out<U7>, U4);
    assert_type_eq!(CeilDivBy2Out<U8>, U4);
}


// A type operator for doubling Self
pub trait Double {
    type Output;
}

impl Double for UTerm {
    type Output = UTerm;
}

impl<U,B> Double for UInt<U,B> {
    type Output = UInt<UInt<U,B>,B0>;
}

// Alias for the result of Double.
pub type DoubleOut<N> = <N as Double>::Output;


// A type operator. The Output is False if Self is UTerm and True if
// Self is some nonzero UInt.
pub trait IsNonZero {
    type Output: Bit;
}

impl IsNonZero for UTerm {
    type Output = False;
}

impl<U,B> IsNonZero for UInt<U,B> {
    type Output = True;
}

// Alias for the result of IsNonZero
pub type IsNonZeroOut<N> = <N as IsNonZero>::Output;
