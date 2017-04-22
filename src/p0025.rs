use std::cmp;

fn add(dst: &mut Vec<usize>, src: &Vec<usize>) {
    let mut carry = 0;
    let overlap = cmp::min(dst.len(), src.len());
    for index in 0..overlap {
        let sum = dst[index] + src[index] + carry;
        dst[index] = sum % 10;
        carry = sum / 10;
    }
    for index in overlap..src.len() {
        let sum = src[index] + carry;
        dst.push(sum % 10);
        carry = sum / 10;
    }
    for index in overlap..dst.len() {
        let sum = dst[index] + carry;
        dst[index] = sum % 10;
        carry = sum / 10;
    }
    while carry != 0 {
        dst.push(carry % 10);
        carry = carry / 10;
    }
}

pub fn solve() -> usize {
    let mut index = 2;
    let mut a = vec![1];
    let mut b = vec![1];

    loop {
        index += 1;
        add(&mut b, &a);
        if b.len() >= 1000 {
            break;
        }

        index += 1;
        add(&mut a, &b);
        if a.len() >= 1000 {
            break;
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut a = vec![];
        let b = vec![];
        add(&mut a, &b);
        assert_eq!(a, []);
        assert_eq!(b, []);

        let mut a = vec![];
        let b = vec![1];
        add(&mut a, &b);
        assert_eq!(a, [1]);
        assert_eq!(b, [1]);

        let mut a = vec![1];
        let b = vec![];
        add(&mut a, &b);
        assert_eq!(a, [1]);
        assert_eq!(b, []);

        let mut a = vec![5];
        let b = vec![5];
        add(&mut a, &b);
        assert_eq!(a, [0, 1]);
        assert_eq!(b, [5]);

        let mut a = vec![1, 0, 9];
        let b = vec![9, 9];
        add(&mut a, &b);
        assert_eq!(a, [0, 0, 0, 1]);
        assert_eq!(b, [9, 9]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(4782, solve());
    }
}
