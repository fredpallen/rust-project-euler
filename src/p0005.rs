use gcd;

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
