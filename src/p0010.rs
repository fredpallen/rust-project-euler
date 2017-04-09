use primes_to;

pub fn solve() -> usize {
    return primes_to(2e6 as usize - 1).iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(142913828922, solve());
    }
}
