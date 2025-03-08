//! # MathUtils
//! `mathutils` is a collection of utilities for mathematical operations.
//!
//! '''
//! use math::math::gcd;
//! use math::math::is_prime;
//! use math::math::factorial;
//! #Examples
//! let result = gcd(2720, 1890);
//! assert_eq!(result, 10);
//! let result = factorial(5);
//! assert_eq!(result, 120);
//! let result = is_prime(7);
//! assert_eq!(result, true);
//! '''

/// Returns the greatest common divisor of two numbers.
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Returns the greatest common divisor of two numbers.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Returns true if the number is prime.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}