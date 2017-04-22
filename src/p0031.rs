fn count_ways(coins: &[u64], change: u64) -> u64 {
    match coins.len() {
        0 => if change == 0 { 1 } else { 0 },
        1 => if change % coins[0] == 0 { 1 } else { 0 },
        _ => {
            let (value, rest) = coins.split_last().unwrap();
            (0..)
                .map(|k| k * value)
                .take_while(|k| *k <= change)
                .map(|k| count_ways(rest, change - k))
                .sum()
        },
    }
}

pub fn solve() -> u64 {
    let coins = [1, 2, 5, 10, 20, 50, 100, 200];
    count_ways(&coins, 200)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(73682, solve());
    }
}
