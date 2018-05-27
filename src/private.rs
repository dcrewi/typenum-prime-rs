// src/private.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use core::ops::{Add, BitAnd, Mul, Rem, Shl, Sub};
use typenum::bit::B1;
use typenum::consts::{False, True, U0, U1};
use typenum::marker_traits::{Bit, Unsigned};
use typenum::operator_aliases::{Add1, And, GrEq, Mod, Square, Sub1, Sum};
use typenum::type_operators::IsGreaterOrEqual;
use typenum::uint::{UInt, UTerm};


// FIXME: I submitted this integer square root algorithm upstream to
// the typenum repository. Use it when it lands.

pub type Double<N> = <N as Shl<B1>>::Output;

pub trait SquareRoot {
    /// The result of the integer square root.
    type Output;
}

pub type Sqrt<N> = <N as SquareRoot>::Output;

impl SquareRoot for UTerm {
    type Output = UTerm;
}

impl SquareRoot for UInt<UTerm, B1> {
    type Output = UInt<UTerm, B1>;
}

impl<U, Ba, Bb> SquareRoot for UInt<UInt<U, Ba>, Bb>
where
    U: Unsigned,
    Ba: Bit,
    Bb: Bit,
    U: SquareRoot,
    Sqrt<U>: Shl<B1>,
    Double<Sqrt<U>>: Add<B1>,
    Add1<Double<Sqrt<U>>>: Mul,
    Self: IsGreaterOrEqual<Square<Add1<Double<Sqrt<U>>>>>,
    Double<Sqrt<U>>: Add<GrEq<Self, Square<Add1<Double<Sqrt<U>>>>>>,
{
    type Output = Sum<Double<Sqrt<U>>, GrEq<Self, Square<Add1<Double<Sqrt<U>>>>>>;
}


/// **Type operator** for nonzero-testing.
pub trait IsNonZero: Unsigned {
    type Output: Bit;
}

impl IsNonZero for UTerm {
    type Output = False;
}

impl<U: Unsigned, B: Bit> IsNonZero for UInt<U,B> {
    type Output = True;
}

pub type NonZero<N> = <N as IsNonZero>::Output;


/// **Type operator** for trial division up to a limit.
pub trait TrialDivisionUpTo<N> {
    /// False if Self is divisible by any number between 2 and N
    /// inclusive (meaning Self is composite), otherwise True (meaning
    /// Self is prime). Defined to be True when Self is U1 to
    /// establish the base case of the recursive algorithm.
    type Output;
}

// algorithm:
// trial_division_up_to(1) := true.
// trial_division_up_to(n) := (self % n != 0) && trial_division_up_to(n-1).

impl<N> TrialDivisionUpTo<UInt<UTerm,B1>> for N {
    type Output = True;
}

impl<N,U,Ba,Bb> TrialDivisionUpTo<UInt<UInt<U,Ba>,Bb>> for N
    where U: Unsigned,
          Ba: Bit,
          Bb: Bit,
          UInt<UInt<U,Ba>,Bb>: Sub<B1>,
          N: Rem<UInt<UInt<U,Ba>,Bb>>,
          Mod<N,UInt<UInt<U,Ba>,Bb>>: IsNonZero,
          NonZero<Mod<N,UInt<UInt<U,Ba>,Bb>>>: BitAnd<<N as TrialDivisionUpTo<Sub1<UInt<UInt<U,Ba>,Bb>>>>::Output>,
          N: TrialDivisionUpTo<Sub1<UInt<UInt<U,Ba>,Bb>>>,

{
    type Output = And<NonZero<Mod<N, UInt<UInt<U,Ba>,Bb>>>,
                      <N as TrialDivisionUpTo<Sub1<UInt<UInt<U,Ba>,Bb>>>>::Output>;
}


pub trait PrivateIsPrime: Unsigned {
    type Output: Bit;
}

impl PrivateIsPrime for U0 {
    type Output = False;
}

impl PrivateIsPrime for U1 {
    type Output = False;
}

impl<U,Ba,Bb> PrivateIsPrime for UInt<UInt<U,Ba>,Bb>
    where U: Unsigned,
          Ba: Bit,
          Bb: Bit,
          Self: Unsigned + SquareRoot + TrialDivisionUpTo<Sqrt<Self>>,
          <Self as TrialDivisionUpTo<Sqrt<Self>>>::Output: Bit {
    type Output = <Self as TrialDivisionUpTo<Sqrt<Self>>>::Output;
}
