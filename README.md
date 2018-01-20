# `typenum-prime`

This is a rust crate that provides a marker trait for primality of
type-level integers from the `typenum` crate. Unsigned integers up to
1024 are marked by a build script which does a super-naive Sieve of
Eratosthenes. Pull requests to extend it to all integers are welcome.

## Usage

The crate is published on https://crates.io/ as `typenum-prime`. Add
it to your `Cargo.toml` in the usual way.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

I would prefer it if you included language to this effect in the
commit message of your first pull request.


## Todo

- Extend the provided impl to all unsigned integers using type-level
  operators. I expect the result to be slow.
