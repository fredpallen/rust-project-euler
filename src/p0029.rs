use std::collections::HashSet;

pub fn solve() -> usize {
    let mut visited = vec![false; 101];
    let mut sum = 0;
    for a in 2..101 {
        if visited[a] {continue;}
        let mut uniques: HashSet<usize> = HashSet::new();
        let mut power = 1;
        let mut value = a;
        while value <= 100 {
            visited[value] = true;
            uniques.extend((2..101).map(|n| power * n));
            value *= a;
            power += 1;
        }
        sum += uniques.len();
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(9183, solve());
    }
}
