use std::cmp;
use std::iter;

fn multiply_and_add_shifted(
        output: &mut Vec<usize>,
        input: &Vec<usize>,
        factor: usize,
        shift: usize) {
    let starting_output_length = output.len();
    if shift > starting_output_length {
        output.extend(iter::repeat(0).take(shift - starting_output_length));
    }
    let mut carry = 0;
    let overlap_count =
        if starting_output_length < shift {0}
        else {
            cmp::min(starting_output_length - shift, input.len())
        };
    for i in 0..overlap_count {
        let value = (factor * input[i]) + output[i + shift] + carry;
        output[i + shift] = value % 10;
        carry = value / 10;
    }
    if input.len() > overlap_count {
        for i in overlap_count..input.len() {
            let value = (factor * input[i]) + carry;
            output.push(value % 10);
            carry = value / 10;
        }
    } else {
        for i in (shift + overlap_count)..output.len() {
            let value = output[i] + carry;
            output[i] = value % 10;
            carry = value / 10;
            if carry == 0 {break;}
        }
    }
    while carry != 0 {
        output.push(carry % 10);
        carry /= 10;
    }
}

fn multiply(v1: &Vec<usize>, v2: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    for (index, digit) in v1.iter().enumerate() {
        multiply_and_add_shifted(&mut result, &v2, *digit, index);
    }
    result
}

pub fn solve() -> usize {
    let mut n = 1000;
    let mut result = vec![1];
    let mut x = vec![2];
    while n != 0 {
        if n % 2 != 0 {
            result = multiply(&result, &x);
        }
        x = multiply(&x, &x);
        n /= 2;
    }
    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(&vec![], &vec![]), []);
        assert_eq!(multiply(&vec![1], &vec![1]), [1]);
        assert_eq!(multiply(&vec![2], &vec![2]), [4]);
        assert_eq!(multiply(&vec![3], &vec![3]), [9]);
        assert_eq!(multiply(&vec![4], &vec![4]), [6, 1]);
        assert_eq!(multiply(&vec![5], &vec![5]), [5, 2]);
        assert_eq!(multiply(&vec![0, 1], &vec![0, 1]), [0, 0, 1]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(1366, solve());
    }
}
