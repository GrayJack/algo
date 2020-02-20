//! Fibonacci sequence algorithms.
use std::fmt::Debug;

use num::{BigUint, One, Zero};

/// A Iterator for the fibonacci sequence.
///
/// # Warning
/// Note that due to using `u128` primitive, you cannot take more than the first 186
/// fibonacci numbers. If you need to go past that, use [`BigFib`] iterator.
///
/// [`BigFib`]: ./struct.BigFib.html
///
/// # Example
/// Print the 100 first fibonacci numbers.
///
/// ```rust
/// # use algos::numerics::fibonacci::BigFib;
/// # fn main() {
/// Fib::new().enumerate().take(100).for_each(|(i, v)| println!("Fib {}: {}", i, v));
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct Fib {
    val: (u128, u128),
}

impl Fib {
    /// Create a new iterator starting at the first fibonacci number, zero.
    pub fn new() -> Self { Self { val: (0, 1) } }

    /// Create a new iterator with the first fibonacci number beeing the `nth` fibonacci
    /// number.
    pub fn at(nth: impl Into<u128>) -> Self { Self { val: _fib(nth.into()) } }
}

impl Iterator for Fib {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.val.0;
        self.val = (self.val.1, self.val.0 + self.val.1);
        Some(next)
    }
}

/// A Iterator for the fibonacci sequence using big numbers.
///
/// # Example
/// Print the 100 first fibonacci numbers.
///
/// ```rust
/// # use algos::numerics::fibonacci::BigFib;
/// # fn main() {
/// BigFib::new().enumerate().take(100).for_each(|(i, v)| println!("Fib {}: {}", i, v));
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct BigFib {
    val: (BigUint, BigUint),
}

impl BigFib {
    /// Create a new iterator starting at the first fibonacci number, zero.
    pub fn new() -> Self { Self { val: (BigUint::zero(), BigUint::one()) } }

    /// Create a new iterator with the first fibonacci number beeing the `nth` fibonacci
    /// number.
    pub fn at(nth: impl Into<BigUint>) -> Self { Self { val: _big_fib(&nth.into()) } }
}

impl Iterator for BigFib {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.val.0.clone();
        self.val = (self.val.1.clone(), &self.val.0 + &self.val.1);
        Some(next)
    }
}

/// Calculate the nth fibonacci number using the classic recursive strategy.
///
/// |   Case    | Time complexity | Space complexity |
/// |:----------|:---------------:|:----------------:|
/// | Best:     | Ω(n²)           | Ω(n)             |
/// | Avrg:     | Θ(n²)           | Θ(n)             |
/// | Worst:    | O(n²)           | O(n)             |
///
/// # Panics
/// This function may panic on debug builds if the internal type (u128) and happens a
/// operation overflow.
pub fn recursive_fibonacci(nth: u128) -> u128 {
    match nth {
        0 => 0,
        1 | 2 => 1,
        _ => recursive_fibonacci(nth - 1) + recursive_fibonacci(nth - 2),
    }
}

/// Calculate the nth fibonacci number using the dynamic programming strategy.
///
/// |   Case    | Time complexity | Space complexity |
/// |:----------|:---------------:|:----------------:|
/// | Best:     | Ω(n)            | Ω(1)             |
/// | Avrg:     | Θ(n)            | Θ(1)             |
/// | Worst:    | O(n)            | O(1)             |
///
/// # Panics
/// This function may panic on debug builds if the internal type (u128) and happens a
/// operation overflow.
pub fn dynamic_fibonacci(nth: u128) -> u128 {
    let (mut a, mut b) = (0, 1);

    for _ in 0..nth {
        let c = a + b;
        a = b;
        b = c;
    }

    a
}

/// Calculate the nth fibonacci number using the fast doubling algorithm.
///
/// |   Case    | Time complexity | Space complexity |
/// |:----------|:---------------:|:----------------:|
/// | Best:     | Ω(log(n))       | Ω(1)             |
/// | Avrg:     | Θ(log(n))       | Θ(1)             |
/// | Worst:    | O(log(n))       | O(1)             |
///
/// # Panics
/// This function may panic on debug builds if the internal type (u128) and happens a
/// operation overflow.
pub fn fast_doubling_fibonacci(nth: u128) -> u128 { _fib(nth).0 }

/// Calculate the fibonacci number at index `nth` using the fast doubling strategy
/// (inner).
fn _fib(nth: u128) -> (u128, u128) {
    if nth == 0 {
        (0, 1)
    } else {
        let (a, b) = _fib(nth / 2);
        let c = a * (b * 2 - a);
        let d = a * a + b * b;

        if nth % 2 == 0 { (c, d) } else { (d, c + d) }
    }
}

/// Calculate the nth fibonacci number using the fast doubling algorithm.
///
/// |   Case    | Time complexity | Space complexity |
/// |:----------|:---------------:|:----------------:|
/// | Best:     | Ω(log(n))       | Ω(1)             |
/// | Avrg:     | Θ(log(n))       | Θ(1)             |
/// | Worst:    | O(log(n))       | O(1)             |
///
/// # Panics
/// This function may panic if `BigUint` type run out of allocation memory.
pub fn big_fast_doubling_fibonacci(nth: impl Into<BigUint>) -> BigUint { _big_fib(&nth.into()).0 }

/// Calculate the fibonacci number at index `nth` using the fast doubling strategy
/// (inner).
fn _big_fib(nth: &BigUint) -> (BigUint, BigUint) {
    if *nth == BigUint::zero() {
        (BigUint::zero(), BigUint::one())
    } else {
        let (a, b) = _big_fib(&(nth / 2u8));
        let c = &a * (&b * 2u8 - &a);
        let d = &a * &a + &b * &b;

        if nth % 2u8 == BigUint::zero() { (c, d) } else { (d.clone(), c + d) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;

    #[test]
    fn recursive_test() {
        let sure = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597];
        let test: Vec<_> = (0..sure.len() as u128).map(recursive_fibonacci).collect();
        assert_eq!(sure, test);
    }

    #[test]
    fn dynamic_test() {
        let sure = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597];
        let test: Vec<_> = (0..sure.len() as u128).map(dynamic_fibonacci).collect();
        assert_eq!(sure, test);
    }

    #[test]
    fn fast_doubling_test() {
        let sure = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597];
        let test: Vec<_> = (0..sure.len() as u128).map(fast_doubling_fibonacci).collect();
        assert_eq!(sure, test);
    }

    #[test]
    fn fast_doubling_bignum_test() {
        let sure: Vec<_> =
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597]
                .iter()
                .map(|x| BigUint::from(*x as u32))
                .collect();

        let test: Vec<_> = (0..sure.len() as u128).map(big_fast_doubling_fibonacci).collect();
        assert_eq!(sure, test);
    }

    #[test]
    fn iterator_test() {
        let sure = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597];
        let test: Vec<_> = Fib::new().take(sure.len()).collect();
        assert_eq!(sure, test);
    }

    #[test]
    fn iterator_bignum_test() {
        let sure: Vec<_> =
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597]
                .iter()
                .map(|x| BigUint::from(*x as u32))
                .collect();

        let test: Vec<_> = BigFib::new().take(sure.len()).collect();
        assert_eq!(sure, test);
    }
}
