use std::collections::HashSet;
use std::iter::FromIterator;

fn proper_divisor_sum(n: &usize) -> usize {
    (1..(n / 2) + 1).filter(|d| n % d == 0).sum()
}

fn is_abundant(n: &usize) -> bool {
    proper_divisor_sum(n) > *n
}

pub fn solve() -> usize {
    let bound = 28123;

    let abundants: HashSet<usize> =
        HashSet::from_iter((1..bound + 1).filter(is_abundant));

    let mut sum = 0;
    'candidates: for n in 1..(bound + 1) {
        for a in &abundants {
            if (n > *a) && abundants.contains(&(n - *a)) {
                // This n is a sum of two abundants.
                continue 'candidates;
            }
        }
        sum += n;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(4179871, solve());
    }
}
