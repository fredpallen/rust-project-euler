fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    for d in (2..).take_while(|d| d * d < n) {
        if n % d == 0 {
            return false;
        }
    }
    true
}

pub fn solve() -> i32 {
    let mut best_run_length = 0;
    let mut result = 0;

    // b must be prime to get a prime for n=0.
    for b in 2..1001 {
        if !is_prime(b) {
            continue;
        }
        for a in -999..1000 {
            for n in 1.. {
                if !is_prime((n * n) + (a * n) + b) {
                    if n > best_run_length {
                        best_run_length = n;
                        result = a * b;
                        println!("a={}, b={}, n={}", a, b, n);
                    }
                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(-59231, solve());
    }
}
