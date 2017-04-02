/// Finds the sum of the even-valued Fibonacci numbers below 4 million.
///
/// ```
/// assert_eq!(project_euler::p0002::solve(), 4613732)
/// ```
pub fn solve() -> u64 {
    let mut sum = 0u64;
    let mut a = 1u64;
    let mut b = 1u64;
    while b < 4000000 {
        if b % 2 == 0 {sum += b;}
        let c = a + b;
        a = b;
        b = c;
    }
    sum
}
