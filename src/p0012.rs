fn divisor_count(n: usize) -> usize {
    let mut count = 2;
    for d in 2..(n / 2 + 1) {
        if n % d == 0 {
            count += 1;
        }
    }
    count
}

fn triangle_divisor_count(n: usize) -> usize {
    let (a, b) = {
        if n % 2 == 0 {
            (n / 2, n + 1)
        } else {
            (n, (n + 1) / 2)
        }
    };
    divisor_count(a) * divisor_count(b)
}

pub fn solve() -> usize {
    let mut n = 0;
    loop {
        n += 1;
        if triangle_divisor_count(n) > 500 {
            break;
        }
    }
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_count() {
        assert_eq!(2, divisor_count(2));
        assert_eq!(2, divisor_count(3));
        assert_eq!(3, divisor_count(4));
        assert_eq!(2, divisor_count(5));
        assert_eq!(4, divisor_count(6));
        assert_eq!(2, divisor_count(7));
        assert_eq!(4, divisor_count(8));
        assert_eq!(3, divisor_count(9));
    }

    #[test]
    fn test_triangle_divisor_count() {
        assert_eq!(4, triangle_divisor_count(4));
        assert_eq!(4, triangle_divisor_count(5));
        assert_eq!(4, triangle_divisor_count(6));
        assert_eq!(6, triangle_divisor_count(7));
    }

    #[test]
    fn test_solve() {
        assert_eq!(76576500, solve());
    }
}
