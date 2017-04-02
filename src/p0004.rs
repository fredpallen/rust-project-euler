/// ```
/// use project_euler::p0004::reverse_digits;
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

/// ```
/// assert_eq!(project_euler::p0004::solve(), 906609);
/// ```
pub fn solve() -> u64 {
    let mut top = 1000;
    let is_three_digits = |n| n >= 100 && n < 1000;
    loop {
        top -= 1;
        let palindrome = 1000 * top + reverse_digits(top);
        for f in 100..999 {
            if palindrome % f == 0 && is_three_digits(palindrome / f) {
                return palindrome;
            }
        }
    }
}
