use std::cmp;
use std::iter;

pub fn solve() -> usize {
    let str_to_usize = |s: &str| -> usize {
        s.parse::<usize>().unwrap()
    };

    // Pad a zero at the beginning of a row.
    let row_str_to_zero_padded = |s: &str| -> Vec<usize> {
        iter::once(0).chain(s.split_whitespace().map(&str_to_usize)).collect()
    };

    let triangle: Vec<Vec<usize>> = "
        75
        95 64
        17 47 82
        18 35 87 10
        20 04 82 47 65
        19 01 23 75 03 34
        88 02 77 73 07 63 67
        99 65 04 28 06 16 70 92
        41 41 26 56 83 40 80 70 33
        41 48 72 33 47 32 37 16 94 29
        53 71 44 65 25 43 91 52 97 51 14
        70 11 33 28 77 73 17 78 39 68 17 57
        91 71 52 38 17 14 91 43 58 50 27 29 48
        63 66 04 68 89 53 67 30 73 16 69 87 40 31
        04 62 98 27 23 09 70 98 73 93 38 53 60 04 23"
            .lines()
            .filter(|s| !s.is_empty())
            .map(row_str_to_zero_padded)
            .collect();

    // Zero pad front and back.
    let mut max_sum = vec![0; triangle.len() + 2];
    for (row_index, row) in triangle.iter().enumerate() {
        let row_length = row_index + 1;
        for column_index in (0..row_length).rev() {
            max_sum[column_index + 1] = row[column_index + 1] +
                cmp::max(max_sum[column_index], max_sum[column_index + 1]);
        }
    }
    *max_sum.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(1074, solve());
    }
}
