pub mod p0001;
pub mod p0002;
pub mod p0003;
pub mod p0004;
pub mod p0005;
pub mod p0006;
pub mod p0007;
pub mod p0008;

use std::cmp;

/// ```
/// use project_euler::gcd;
///
/// assert_eq!(0, gcd(0, 0));
/// assert_eq!(0, gcd(1, 0));
/// assert_eq!(0, gcd(0, 1));
/// assert_eq!(1, gcd(1, 1));
/// assert_eq!(1, gcd(2, 1));
/// assert_eq!(1, gcd(1, 2));
/// assert_eq!(2, gcd(2, 2));
/// assert_eq!(1, gcd(2, 3));
/// assert_eq!(1, gcd(3, 2));
/// assert_eq!(2, gcd(2, 4));
/// assert_eq!(2, gcd(4, 2));
/// assert_eq!(2, gcd(4, 6));
/// assert_eq!(2, gcd(6, 4));
/// assert_eq!(3, gcd(6, 9));
/// assert_eq!(3, gcd(9, 6));
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    let mut big = cmp::max(a, b);
    let mut small = cmp::min(a, b);
    if small == 0 {
        return 0;
    }
    while small != 0 {
        let r = big % small;
        big = small;
        small = r;
    }
    big
}

/// ```
/// use project_euler::reverse_digits;
///
/// assert_eq!(0, reverse_digits(0));
/// assert_eq!(1, reverse_digits(1));
/// assert_eq!(9, reverse_digits(9));
/// assert_eq!(1, reverse_digits(10));
/// assert_eq!(11, reverse_digits(11));
/// assert_eq!(21, reverse_digits(12));
/// assert_eq!(1, reverse_digits(100));
/// assert_eq!(101, reverse_digits(101));
/// assert_eq!(201, reverse_digits(102));
/// ```
pub fn reverse_digits(n: u64) -> u64 {
    let mut remainder = n;
    let mut reversed = 0;
    while remainder != 0 {
        reversed *= 10;
        reversed += remainder % 10;
        remainder /= 10;
    }
    reversed
}
