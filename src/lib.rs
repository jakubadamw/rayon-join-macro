//! A wrapper around `rayon::join` that accepts more than 2 and up to 8 closures to be run in parallel.
//!
//! # Examples
//!
//! ```
//! # #[macro_use] extern crate rayon_join_macro;
//! # fn main() {
//! use rayon_join_macro::ConsTuple;
//!
//! fn factorial(n: usize) -> usize {
//!     if n == 0 { 1 } else { n * factorial(n - 1) }
//! }
//!
//! let (a, b, c): (u16, String, usize) = join!(
//!     || (1..=50).sum(),
//!     || "abc".repeat(3),
//!     || factorial(8)
//! );
//! assert_eq!(a, 1275);
//! assert_eq!(b, "abcabcabc");
//! assert_eq!(c, 40320);
//! # }
//! ```

#![allow(clippy::needless_doctest_main)]

pub trait ConsTuple {
    type Flattened;
    fn flattened(self) -> Self::Flattened;
}

macro_rules! cons_tuple_ty {
    ($lhs:ident) => { ($lhs,) };
    ($lhs:ident, $($tail:ident),*) => { ($lhs, cons_tuple_ty!($($tail),*)) };
}

macro_rules! impl_cons_tuple {
    ($lhs:ident) => {
        #[allow(non_snake_case)]
        impl<$lhs> ConsTuple for ($lhs,) {
            type Flattened = ($lhs,);
            fn flattened(self) -> Self::Flattened {
                self
            }
        }
    };

    ($lhs:ident, $($tail:ident),+) => {
        #[allow(non_snake_case)]
        impl<$lhs, $($tail),+> ConsTuple for cons_tuple_ty!($lhs, $($tail),+) {
            type Flattened = ($lhs, $($tail),+);
            fn flattened(self) -> Self::Flattened {
                let (lhs, rhs) = self;
                let ($($tail, )+): ($($tail, )+) = rhs.flattened();
                (lhs, $($tail),+)
            }
        }
    };
}

impl_cons_tuple!(T1);
impl_cons_tuple!(T1, T2);
impl_cons_tuple!(T1, T2, T3);
impl_cons_tuple!(T1, T2, T3, T4);
impl_cons_tuple!(T1, T2, T3, T4, T5);
impl_cons_tuple!(T1, T2, T3, T4, T5, T6);
impl_cons_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_cons_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);

#[doc(hidden)]
#[macro_export]
macro_rules! join_ {
    ($e:expr) => { ($e(),) };
    ($e:expr, $($tail:expr),+) => { rayon::join($e, || join_!($($tail),+)) };
}

/// A wrapper around `rayon::join` that accepts more than 2 and up to 8 closures to be run in parallel.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rayon_join_macro;
/// # fn main() {
/// use rayon_join_macro::ConsTuple;
/// let (a, b, c) = join!(
///     || (1..=100).count(),
///     || (101..=200).count(),
///     || (201..=300).count()
/// );
/// assert_eq!(a + b + c, 300);
/// # }
/// ```
#[macro_export]
macro_rules! join {
    ($($e:expr),+) => { join_!($($e),+).flattened() };
}

#[test]
fn test_join() {
    let (a, b, c) = join!(
        || (1..=100).count(),
        || (101..=200).count(),
        || (201..=300).count()
    );
    assert_eq!(a + b + c, 300);
}
