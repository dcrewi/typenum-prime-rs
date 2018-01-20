// src/lib.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate typenum;

use typenum::consts::*;


/// Marker trait for prime UInts.
pub trait Prime {}

include!(concat!(env!("OUT_DIR"), "/build-script-sieved.rs"));
