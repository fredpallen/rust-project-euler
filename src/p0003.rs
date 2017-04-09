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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(6857, solve());
    }
}
