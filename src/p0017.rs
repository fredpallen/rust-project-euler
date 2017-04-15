pub fn solve() -> usize {
    let array_char_count = |a: &[&str]| -> usize {
        a.iter().map(|s| s.len()).sum()
    };

    let digits = array_char_count(&[
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"]);

    let tens = array_char_count(&[
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen"]);

    let decades = array_char_count(&[
        "twenty",
        "thirty",
        "forty",
        "fifty",
        "sixty",
        "seventy",
        "eighty",
        "ninety"]);

    let below_one_hundred =
        (9 * digits) +
        tens +
        (10 * decades);

    let below_one_thousand =
        (10 * below_one_hundred) +
        (100 * digits) +
        (900 * "hundred".len()) +
        ((900 - 9) * "and".len());

    below_one_thousand + "one".len() + "thousand".len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(21124, solve());
    }
}
