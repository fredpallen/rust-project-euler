fn factorial(n: usize) -> usize {
    (1..(n + 1)).product()
}

pub fn solve() -> usize {
    let mut n = 1_000_000 - 1;
    let mut selections = vec![];
    while n != 0 {
        for k in 1.. {
            if factorial(k) > n {
                let times = n / factorial(k - 1);
                selections.push(times);
                n -= times * factorial(k - 1);
                break;
            }
        }
    }
    let mut digits: Vec<usize> = (0..10).collect();
    let mut permutation = vec![];
    for s in selections {
        permutation.push(digits.remove(s));
    }
    permutation.extend(digits);
    let mut result = 0;
    for d in permutation {
        result *= 10;
        result += d;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(6, factorial(3));
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn test_solve() {
        assert_eq!(2783915460, solve());
    }
}
