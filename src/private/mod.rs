// src/private/mod.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use typenum::bit::{Bit, B1};
use typenum::consts::False;
use typenum::operator_aliases::Length;
use typenum::type_operators::Len;
use typenum::uint::{UInt, UTerm};

use self::misc::{CeilDivBy2, CeilDivBy2Out};
use self::trial_division::TrialDivisionTreeBranch0;

mod misc;
mod reduction;
mod trial_division;


// A type operator for primality testing.
pub trait PrivateIsPrime {
    type Output: Bit;
}

// 0 is not considered prime
impl PrivateIsPrime for UTerm {
    type Output = False;
}

// 1 is not considered prime
impl PrivateIsPrime for UInt<UTerm, B1> {
    type Output = False;
}

// Numbers greater than 1 apply a test using trial division by
// integers up to the next power of 2 greater than sqrt(n).
impl<U,Ba,Bb> PrivateIsPrime for UInt<UInt<U,Ba>,Bb>
    where Self: Len,
          Length<Self>: CeilDivBy2,
          Self: TrialDivisionTreeBranch0<CeilDivBy2Out<Length<Self>>, UTerm> {
    type Output = <Self as TrialDivisionTreeBranch0<CeilDivBy2Out<Length<Self>>, UTerm>>::Output;
}
