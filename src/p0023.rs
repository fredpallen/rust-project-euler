fn proper_divisor_sum(n: &usize) -> usize {
    let mut sum = 1;
    for k in (2..).take_while(|k| k * k <= *n) {
        if n % k == 0 {
            sum += k;
            let q = n / k;
            if q != k {
                sum += q;
            }
        }
    }
    sum
}

fn is_abundant(n: &usize) -> bool {
    proper_divisor_sum(n) > *n
}

pub fn solve() -> usize {
    let bound = 28123;

    let abundants: Vec<usize> = (1..(bound + 1)).filter(is_abundant).collect();

    let mut is_sum = vec![false; bound + 1];
    for a in &abundants {
        for b in &abundants {
            if (b > a) || (a + b > bound) {
                break;
            }
            is_sum[a + b] = true;
        }
    }

    (1..(bound + 1)).filter(|n| !is_sum[*n]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(4179871, solve());
    }
}
