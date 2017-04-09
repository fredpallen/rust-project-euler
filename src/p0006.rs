pub fn solve() -> u64 {
    let n = 100;
    let s1 = n * (n + 1) / 2;
    let s2 = n * (n + 1) * (2 * n + 1) / 6;
    s1 * s1 - s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(25164150, solve());
    }
}
