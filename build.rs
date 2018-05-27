// build.rs
//
// Copyright (c) 2018 David Creswick
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;


const MAX_SIEVED: usize = 1024;


pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build-script-generated-tests.rs");
    write_tests_to(dest_path).unwrap();
}

fn write_tests_to<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let mut f = File::create(path)?;
    let is_prime = simplistic_sieve(MAX_SIEVED);
    writeln!(f, "#[cfg(test)]")?;
    writeln!(f, "mod generated_tests {{")?;
    writeln!(f, "    use typenum::Bit;")?;
    writeln!(f, "    use typenum::consts::*;")?;
    writeln!(f, "    use super::IsPrime;")?;
    writeln!(f, "    #[test]")?;
    writeln!(f, "    fn small_primes() {{")?;
    writeln!(f, "        // primes")?;
    for (n, &is_prime) in is_prime.iter().enumerate() {
        if is_prime {
            writeln!(f, "        assert!(<U{} as IsPrime>::Output::to_bool());", n)?;
        }
    }
    writeln!(f, "")?;
    writeln!(f, "        // composites")?;
    for (n, &is_prime) in is_prime.iter().enumerate() {
        if !is_prime {
            writeln!(f, "        assert!(!<U{} as IsPrime>::Output::to_bool());", n)?;
        }
    }
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;
    Ok(())
}


fn simplistic_sieve(limit: usize) -> Vec<bool> {
    let mut is_prime = vec![true; 1+limit];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..limit+1 {
        if is_prime[i] {
            for j in 2.. {
                if i*j > limit {
                    break;
                } else {
                    is_prime[i*j] = false;
                }
            }
        }
    }
    return is_prime;
}
