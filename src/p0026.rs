fn get_cycle_length(d: usize) -> usize {
    let mut remainders = vec![1];
    let mut r = 1;
    loop {
        r *= 10;
        r %= d;
        if let Some(index) = remainders.iter().position(|k| *k == r) {
            return remainders.len() - index;
        }
        remainders.push(r);
    }
}

pub fn solve() -> usize {
    let mut longest_cycle = 1;
    let mut result = 1;
    for d in 2..1000 {
        let cycle_length = get_cycle_length(d);
        if cycle_length > longest_cycle {
            longest_cycle = cycle_length;
            result = d;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(983, solve());
    }
}
