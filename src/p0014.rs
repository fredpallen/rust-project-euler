pub fn solve() -> usize {
    let mut lengths = vec![0, 1];
    for n in 2..1_000_000 {
        let mut length = 0;
        let mut r = n;
        while r >= n {
            length += 1;
            if r % 2 == 0 {
                r /= 2;
            } else {
                r = 3 * r + 1;
            }
        }
        let rest = lengths[r];
        lengths.push(length + rest);
    }
    lengths.iter().enumerate().max_by_key(|pair| pair.1).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(837799, solve());
    }
}
