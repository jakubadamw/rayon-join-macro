# rayon-join-macro

![Build status](https://github.com/jakubadamw/rayon-join-macro/workflows/Build/badge.svg)
[![crates.io](https://img.shields.io/crates/v/rayon-join-macro.svg)](https://crates.io/crates/rayon-join-macro)
[![docs.rs](https://docs.rs/rayon-join-macro/badge.svg)](https://docs.rs/rayon-join-macro/latest)
[![Code coverage](https://codecov.io/github/jakubadamw/rayon-join-macro/branch/master/graph/badge.svg)](https://codecov.io/gh/jakubadamw/rayon-join-macro/branch/master)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A wrapper around `rayon::join` that accepts more than 2 and up to 8 closures to be run in parallel.

[Documentation](https://docs.rs/rayon-join-macro/latest/)

```rust
use rayon_join_macro::join;

fn main() {
    fn factorial(n: usize) -> usize {
        if n == 0 { 1 } else { n * factorial(n - 1) }
    }

    let (a, b, c): (u16, String, usize) = join!(
        || (1..=50).sum(),
        || "abc".repeat(3),
        || factorial(8)
    );

    assert_eq!(a, 1275);
    assert_eq!(b, "abcabcabc");
    assert_eq!(c, 40320);
}
```
