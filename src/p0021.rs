fn proper_divisor_sum(n: usize) -> usize {
    (1..(n / 2) + 1).filter(|d| n % d == 0).sum()
}

pub fn solve() -> usize {
    (1..10000)
        .filter(
            |n| {
                let s = proper_divisor_sum(*n);
                (s < 10000) && (s != *n) && (proper_divisor_sum(s) == *n)
            },
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_divisor_sum() {
        assert_eq!(0, proper_divisor_sum(1));
        assert_eq!(1, proper_divisor_sum(2));
        assert_eq!(1, proper_divisor_sum(3));
        assert_eq!(3, proper_divisor_sum(4));
        assert_eq!(1, proper_divisor_sum(5));
        assert_eq!(6, proper_divisor_sum(6));
        assert_eq!(284, proper_divisor_sum(220));
        assert_eq!(220, proper_divisor_sum(284));
    }

    #[test]
    fn test_solve() {
        assert_eq!(31626, solve());
    }
}
