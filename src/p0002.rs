pub fn solve() -> u64 {
    let mut sum = 0u64;
    let mut a = 1u64;
    let mut b = 1u64;
    while b < 4000000 {
        if b % 2 == 0 {sum += b;}
        let c = a + b;
        a = b;
        b = c;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(4613732, solve());
    }
}
