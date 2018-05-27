// test/larger_primes.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![recursion_limit="256"]
extern crate typenum;
extern crate typenum_prime;

use typenum::consts::*;
use typenum::bit::Bit;
use typenum::operator_aliases::{Shleft, Sub1};

use typenum_prime::IsPrime;


// Test mersenne primes only because they are easy to succinctly
// write.


type MersenneNumber<N> = Sub1<Shleft<U1, N>>;


#[test]
fn test_larger_mersenne_primes() {
    assert!(<MersenneNumber<U13> as IsPrime>::Output::to_bool()); // 8_191
    // FIXME: these are prime, but too expensive to compute using the current algorithm.
    //assert!(<MersenneNumber<U17> as IsPrime>::Output::to_bool()); // 131_071
    //assert!(<MersenneNumber<U19> as IsPrime>::Output::to_bool()); // 524_287
    //assert!(<MersenneNumber<U31> as IsPrime>::Output::to_bool()); // 2_147_483_647
}
