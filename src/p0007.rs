/// ```
/// assert_eq!(project_euler::p0007::solve(), 104743)
/// ```
pub fn solve() -> usize {
    let n = 10001;

    // For n > 6, the nth prime is less than n(ln(n) + ln(ln(n)))
    let bound = |n: f64| n * (n.ln() + n.ln().ln());
    let limit = bound(n as f64) as usize;

    let mut is_known_composite = vec![false; limit + 1];
    let mut prime = 3;
    let mut count = 2;

    while count < n {
        count += 1;
        let mut c = prime * prime;
        while c < limit {
            is_known_composite[c] = true;
            c += 2 * prime;
        }
        prime += 2;
        while is_known_composite[prime] {
            prime += 2;
        }
    }

    prime
}
