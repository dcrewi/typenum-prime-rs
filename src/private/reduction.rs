// src/private/reduction.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use core::ops::Add;

use typenum::bit::{B0, B1};
use typenum::uint::{Unsigned, UInt, UTerm};
use typenum::marker_traits::NonZero;
use typenum::operator_aliases::Add1;

use super::misc::{Double, DoubleOut};


// A special struct representing the result of underflowed
// subtraction. (Right-hand side was greater than left-hand side.)
pub struct Underflowed;


// A type operator. If Self is a type-level integer, it left-shifts
// Self by 1 and sets the least significant bit to B. If Self is
// Underflowed, the Output is Underflowed.
pub trait PushBit<B> {
    type Output;
}

impl PushBit<B0> for UTerm {
    type Output = UTerm;
}

impl PushBit<B1> for UTerm {
    type Output = UInt<UTerm,B1>;
}

impl<Ul,Bl,B> PushBit<B> for UInt<Ul,Bl> {
    type Output = UInt<UInt<Ul,Bl>,B>;
}

impl<B> PushBit<B> for Underflowed {
    type Output = Underflowed;
}

// Alias for the result of PushBit<B>.
type PushBitOut<U,B> = <U as PushBit<B>>::Output;


// A type operator. If Self is greater or equal to RHS, the Output is
// their difference. If RHS is greater than Self, the Output is
// Underflowed.
pub trait TrySub<RHS> {
    type Output;
}

impl<L> TrySub<UTerm> for L {
    type Output = L;
}

impl<Ur,Br> TrySub<UInt<Ur,Br>> for UTerm {
    type Output = Underflowed;
}

impl<Ul,Bl,Ur> TrySub<UInt<Ur,B0>> for UInt<Ul,Bl>
    where Ul: TrySub<Ur>,
          TrySubOut<Ul,Ur>: PushBit<Bl> {
    type Output = PushBitOut<TrySubOut<Ul,Ur>, Bl>;
}

impl<Ul,Ur> TrySub<UInt<Ur,B1>> for UInt<Ul,B1>
    where Ul: TrySub<Ur>,
          TrySubOut<Ul,Ur>: PushBit<B0> {
    type Output = PushBitOut<TrySubOut<Ul,Ur>, B0>;
}

impl<Ul,Ur> TrySub<UInt<Ur,B1>> for UInt<Ul,B0>
    where Ur: Add<B1>,
          Ul: TrySub<Add1<Ur>>,
          TrySubOut<Ul,Add1<Ur>>: PushBit<B1> {
    type Output = PushBitOut<TrySubOut<Ul,Add1<Ur>>, B1>;
}

// Alias for the result of TrySub.
type TrySubOut<L,R> = <L as TrySub<R>>::Output;


// A type operator. If Self is a type-level integer, the Output is
// Self. If Self is `Underflowed`, the Output is D.
pub trait ReplaceUnderflow<D> {
    type Output;
}

impl<D> ReplaceUnderflow<D> for Underflowed {
    type Output = D;
}

impl<D> ReplaceUnderflow<D> for UTerm {
    type Output = Self;
}

impl<U,B,D> ReplaceUnderflow<D> for UInt<U,B> {
    type Output = Self;
}

// Alias for the result of ReplaceUnderflow<D>.
type ReplaceUnderflowOut<U, D> = <U as ReplaceUnderflow<D>>::Output;


// A type operator. Try to subtract M from Self. On underflow, output
// Self unchanged. On successful subtraction, output the difference.
//
// This is essentially
//     if Self > M { Self - M } else { Self }
// but without recursing twice (once for the comparison, then a second
// time for the actual subtraction).
pub trait ReduceOnce<M> {
    type Output;
}

impl<N,M> ReduceOnce<M> for N
    where M: NonZero,
          N: TrySub<M>,
          TrySubOut<N,M>: ReplaceUnderflow<N> {
    type Output = ReplaceUnderflowOut<TrySubOut<N,M>, N>;
}

// Alias for the result of ReduceOnce.
type ReduceOnceOut<N,M> = <N as ReduceOnce<M>>::Output;

#[test]
fn test_reduce_once() {
    use typenum::consts::*;
    assert_type_eq!(U1, ReduceOnceOut<U5,U4>);
    assert_type_eq!(U3, ReduceOnceOut<U3,U4>);
    assert_type_eq!(U3, ReduceOnceOut<U3,U5>);
    assert_type_eq!(U3, ReduceOnceOut<U8,U5>);
    assert_type_eq!(U8, ReduceOnceOut<U13,U5>);
}


// A type operator for remainder of division.
pub trait Reduce<M> {
    type Output;
}

// If Self is zero, the result is always zero.
impl<Um,Bm> Reduce<UInt<Um,Bm>> for UTerm {
    type Output = UTerm;
}

// It helps a lot to remember that ReduceOnce<M> is subtraction by M
// once, unless M is larger than Self.

// (2*n+0)%m == reduce_once(2*(n%m))
impl<Un,M> Reduce<M> for UInt<Un,B0>
    where Un: Unsigned,
          M: Unsigned + NonZero,
          Un: Reduce<M>,
          ReduceOut<Un,M>: Double,
          DoubleOut<ReduceOut<Un,M>>: ReduceOnce<M> {
    type Output = ReduceOnceOut<DoubleOut<ReduceOut<Un,M>>,M>;
}

// (2*n+1)%m == reduce_once(2*(n%m)+1)
impl<Un,M> Reduce<M> for UInt<Un,B1>
    where Un: Unsigned,
          M: Unsigned + NonZero,
          Un: Reduce<M>,
          ReduceOut<Un,M>: Double,
          DoubleOut<ReduceOut<Un,M>>: Add<B1>,
          Add1<DoubleOut<ReduceOut<Un,M>>>: ReduceOnce<M> {
    type Output = ReduceOnceOut<Add1<DoubleOut<ReduceOut<Un,M>>>, M>;
}

// Alias for the result of Reduce<M>.
pub type ReduceOut<N,M> = <N as Reduce<M>>::Output;

#[test]
fn test_reduce() {
    use typenum::consts::*;
    assert_type_eq!(U1, ReduceOut<U5,U4>);
    assert_type_eq!(U3, ReduceOut<U3,U4>);
    assert_type_eq!(U3, ReduceOut<U3,U5>);
    assert_type_eq!(U3, ReduceOut<U8,U5>);
    assert_type_eq!(U3, ReduceOut<U13,U5>);
    assert_type_eq!(U3, ReduceOut<U18,U5>);
}
