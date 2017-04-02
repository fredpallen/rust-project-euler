use std::cmp;

/// ```
/// use project_euler::p0005::gcd;
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
/// assert_eq!(project_euler::p0005::solve(), 232792560)
/// ```
pub fn solve() -> u64 {
    let mut result = 1;
    for f in 2..20 {
        result *= f / gcd(f, result);
    }
    result
}
