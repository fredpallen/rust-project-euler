pub fn solve() -> usize {
    // Takes 40 moves: 20 right and 20 down.
    // Solution is nCr(40, 20).
    let mut result = 1;
    for i in 1..21 {
        result *= 20 + i;
        result /= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(137846528820, solve());
    }
}
