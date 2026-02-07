//! Minimal constant_time_eq 0.4.2-compatible crate (edition 2021) for older Cargo.

#![cfg_attr(not(feature = "std"), no_std)]

/// Constant-time byte comparison.
#[inline]
#[must_use]
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Constant-time comparison for fixed-size arrays.
#[inline]
#[must_use]
pub fn constant_time_eq_n<const N: usize>(a: &[u8; N], b: &[u8; N]) -> bool {
    constant_time_eq(a.as_slice(), b.as_slice())
}

#[inline]
#[must_use]
pub fn constant_time_eq_16(a: &[u8; 16], b: &[u8; 16]) -> bool {
    constant_time_eq_n(a, b)
}

#[inline]
#[must_use]
pub fn constant_time_eq_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    constant_time_eq_n(a, b)
}

#[inline]
#[must_use]
pub fn constant_time_eq_64(a: &[u8; 64], b: &[u8; 64]) -> bool {
    constant_time_eq_n(a, b)
}
