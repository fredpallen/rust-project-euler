pub fn solve() -> u64 {
    for m in 1..1001 {
        for n in 1..m {
            let a = 2 * m * n;
            let b = m * m - n * n;
            let c = m * m + n * n;
            if a + b + c == 1000 {
                return a * b * c;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(31875000, solve());
    }
}
