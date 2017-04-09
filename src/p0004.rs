use reverse_digits;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(906609, solve());
    }
}
