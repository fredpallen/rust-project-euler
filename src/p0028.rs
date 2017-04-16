pub fn solve() -> usize {
    let mut n = 1;
    let mut stride = 2;
    let mut sum = 1;

    while stride < 1001 {
        for _ in 0..4 {
            n += stride;
            sum += n;
        }
        stride += 2;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(669171001, solve());
    }
}
