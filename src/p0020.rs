pub fn solve() -> usize {
    let mut digits = vec![1];
    for n in 2..101 {
        let mut carry = 0;
        for value in &mut digits {
            let product = (n * (*value)) + carry;
            *value = product % 10;
            carry = product / 10;
        }
        while carry != 0 {
            digits.push(carry % 10);
            carry /= 10;
        }
    }
    digits.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(648, solve());
    }
}
