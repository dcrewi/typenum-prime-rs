#!/usr/bin/env python3
#
# util/generate_small_constants_test.py
#
# Copyright (c) 2018 David Creswick
#
# Licensed under the Apache License, Version 2.0
# <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
# license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. All files in the project carrying such notice may not be copied,
# modified, or distributed except according to those terms.

# simple sieve of Eratosthenes
N = 1024
is_prime = [True]*(N+1)
is_prime[0] = False
is_prime[1] = False
for n in range(2, N+1):
    if is_prime[n]:
        for m in range(2*n, N+1, n):
            is_prime[m] = False

print("// src/test_small_constants.rs")
print("//")
print("// Copyright (c) 2018 David Creswick")
print("//")
print("// Licensed under the Apache License, Version 2.0")
print("// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT")
print("// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your")
print("// option. All files in the project carrying such notice may not be copied,")
print("// modified, or distributed except according to those terms.")
print()
print("// DO NOT EDIT THIS FILE DIRECTLY!");
print("// This file is the output of util/generate_small_constant_test.py");
print("use typenum::*;")
print("use super::*;")
print()
for (test_name, b) in [("prime", True),
                       ("composite", False)]:
    print()
    print("#[test]")
    print("fn test_%s() {"%test_name)
    for n in range(N+1):
        if is_prime[n] == b:
            #print("    assert!(%s<U%d as IsPrime>::Output::to_bool());"%(s,n))
            print("    assert_type_eq!(<U%d as IsPrime>::Output, %s);"%(n,b))
    print("}")
