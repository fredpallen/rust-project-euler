use gcd;

pub fn solve() -> u64 {
    let mut result = 1;
    for f in 2..20 {
        result *= f / gcd(f, result);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(232792560, solve());
    }
}
