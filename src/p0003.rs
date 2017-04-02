/// ```
/// assert_eq!(project_euler::p0003::solve(), 6857);
/// ```
pub fn solve() -> u64 {
    let mut n = 600851475143u64;
    let mut f = 1u64;

    while n != 1 {
        f += 1;
        while n % f == 0 {
            n /= f;
        }
    }

    f
}
