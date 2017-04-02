/// ```
/// assert_eq!(project_euler::p0006::solve(), 25164150);
/// ```
pub fn solve() -> u64 {
    let n = 100;
    let s1 = n * (n + 1) / 2;
    let s2 = n * (n + 1) * (2 * n + 1) / 6;
    s1 * s1 - s2
}
