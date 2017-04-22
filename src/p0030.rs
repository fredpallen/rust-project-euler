fn power_digit_sum(n: usize, power: u32) -> usize {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(power);
        n /= 10;
    }
    sum
}

pub fn solve() -> usize {
    let power = 5;
    let max_digit_power = 9_usize.pow(power);
    let max_digits =
        (1..).skip_while(
                |k| 10_usize.pow(*k) < (*k as usize) * max_digit_power)
            .next()
            .unwrap();
    (10..10_usize.pow(max_digits))
        .filter(|k| *k == power_digit_sum(*k, power))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(443839, solve());
    }
}
