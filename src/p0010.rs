use primes_to;

/// ```
/// assert_eq!(project_euler::p0010::solve(), 142913828922);
/// ```
pub fn solve() -> usize {
    return primes_to(2e6 as usize - 1).iter().sum();
}
