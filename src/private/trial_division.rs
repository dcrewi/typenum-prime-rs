// src/private/trial_division.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use core::ops::Sub;

use typenum::bit::{Bit, B1};
use typenum::consts::{True, False};
use typenum::operator_aliases::Sub1;
use typenum::uint::{UInt, UTerm};

use super::misc::{Double, DoubleOut, IsNonZero, IsNonZeroOut};
use super::reduction::{Reduce, ReduceOut};
use super::PrivateIsPrime;


// The broad idea of the trial division algorithm:
//
// Search a binary tree depth-first. Label the branches of the tree
// with a 0 or 1 bit. Label the leaves of the tree by integers
// constructed by the path taken down the tree. Do a division test at
// the leaf using those integers. The division test computes whether
// Self is divisible by the leaf integer by checking whether the
// remainder of division is zero.
//
// Additional optimizations:
//
// - Only do the division test if the divisor at the leaf is prime.
//   This is accomplished by calling the whole IsPrime algorithm
//   recursively on the divisor.  (Note that this causes the division
//   test by 0 and 1 to be skipped because they are not considered
//   prime.)
//
// - Exit the tree search early when the first branch taken indicates
//   that Self is composite. (Do a short-circuiting "and" of the
//   results from the two branches of the search tree.)


// A type operator to compute whether Self is divisible by Divisor.
// The test is skipped entirely when DivisorIsPrime is False.
pub trait TrialDivisionTreeLeaf<Divisor,DivisorIsPrime> {
    type Output: Bit;
}

impl<N,D> TrialDivisionTreeLeaf<D,False> for N {
    type Output = True;
}

impl<N,D> TrialDivisionTreeLeaf<D,True> for N
    where N: Reduce<D>,
          ReduceOut<N,D>: IsNonZero {
    type Output = IsNonZeroOut<ReduceOut<N,D>>;
}


// A type operator. If DepthCount is zero, determine whether the
// accumulated Divisor is prime and call TrialDivisionLeaf. If
// DepthCount is positive, decrement it and try the 0 branch, then the
// 1 branch.
pub trait TrialDivisionTreeBranch0<DepthCount, Divisor> {
    type Output: Bit;
}

// the case when Depth is zero: test the divisor for primality and
// pass the result to TrialDivisionTreeLeaf.
impl<D,N> TrialDivisionTreeBranch0<UTerm, D> for N
    where D: PrivateIsPrime,
          N: TrialDivisionTreeLeaf<D, <D as PrivateIsPrime>::Output> {
    type Output = <N as TrialDivisionTreeLeaf<D, <D as PrivateIsPrime>::Output>>::Output;
}


// the case when Depth is nonzero: Decrement the depth, call
// TrialDivisionTreeBranch0 with a 0 bit appended to the divisor, and
// send the result to TrialDivisionTreeBranch1. It will use the result
// of the branch0 to either short-circuit the search or to descend
// with a 1 bit appended to the divisor.
impl<N,Uc,Bc,D> TrialDivisionTreeBranch0<UInt<Uc,Bc>,D> for N
    where UInt<Uc,Bc>: Sub<B1>,
          D: Double,
          N: TrialDivisionTreeBranch0<Sub1<UInt<Uc,Bc>>, DoubleOut<D>>,
          N: TrialDivisionTreeBranch1<Sub1<UInt<Uc,Bc>>,
                                      UInt<D,B1>,
                                      <N as TrialDivisionTreeBranch0<Sub1<UInt<Uc,Bc>>, DoubleOut<D>>>::Output> {
    type Output = <N as TrialDivisionTreeBranch1<Sub1<UInt<Uc,Bc>>,
                                                 UInt<D,B1>,
                                                 <N as TrialDivisionTreeBranch0<Sub1<UInt<Uc,Bc>>, DoubleOut<D>>>::Output>>::Output;
}


pub trait TrialDivisionTreeBranch1<DepthCount, Divisor, ResultFromSiblingBranch> {
    type Output: Bit;
}

// If the sibling branch discovered that Self is composite,
// short-circuit with that result.
impl<N,C,D> TrialDivisionTreeBranch1<C,D,False> for N {
    type Output = False;
}

// If the sibling branch didn't discover that Self is composite,
// continue the search.
impl<N,C,D> TrialDivisionTreeBranch1<C,D,True> for N
    where N: TrialDivisionTreeBranch0<C,D> {
    type Output = <N as TrialDivisionTreeBranch0<C,D>>::Output;
}
