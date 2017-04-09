extern crate num;

use self::num::bigint::ToBigUint;
use self::num::BigUint;
use self::num::One;
use self::num::Zero;

pub trait SafeToBigUint: ToBigUint {
    fn safe_to_biguint(&self) -> BigUint {
        self.to_biguint().unwrap()
    }
}

impl SafeToBigUint for usize {}
impl SafeToBigUint for u8 {}
impl SafeToBigUint for u16 {}
impl SafeToBigUint for u32 {}
impl SafeToBigUint for u64 {}
impl SafeToBigUint for BigUint {}

pub fn sum_consecutive_ints_usize(first: usize, count: usize) -> usize {
    if count == 0 {return 0;}

    // Sum the integers from 1 to count, inclusive.
    let sum_to = |count| count * (count + 1) / 2;

    let below_first = if first == 0 {0} else {sum_to(first - 1)};
    let to_last = sum_to(first + count - 1);
    to_last - below_first
}

pub fn sum_first_ints<T>(count: T) -> BigUint
        where T: SafeToBigUint {
    let _0 = BigUint::zero();
    let _1 = &BigUint::one();
    let _2 = &BigUint::from(2u32);

    let count = &count.safe_to_biguint();
    if *count == _0 {
        _0
    } else {
        (count * (count + _1)) / _2
    }
}

pub fn sum_consecutive_ints<T, U>(first: T, count: U) -> BigUint
        where T: SafeToBigUint, U: SafeToBigUint {
    let _0 = BigUint::zero();
    let _1 = &BigUint::one();

    let first = &first.safe_to_biguint();
    let count = &count.safe_to_biguint();

    if *count == _0 {
        return _0;
    }

    let sum_below_first =
        if *first == _0 {
            _0
        } else {
            sum_first_ints(first - _1)
        };

    let sum_zero_to_last = sum_first_ints(first + count - _1);

    sum_zero_to_last - sum_below_first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_consecutive_ints_usize() {
        assert_eq!(sum_consecutive_ints_usize(0, 0), 0);
        assert_eq!(sum_consecutive_ints_usize(1, 0), 0);
        assert_eq!(sum_consecutive_ints_usize(2, 0), 0);
        assert_eq!(sum_consecutive_ints_usize(0, 1), 0);
        assert_eq!(sum_consecutive_ints_usize(0, 2), 1);
        assert_eq!(sum_consecutive_ints_usize(0, 3), 3);
        assert_eq!(sum_consecutive_ints_usize(0, 4), 6);
        assert_eq!(sum_consecutive_ints_usize(1, 1), 1);
        assert_eq!(sum_consecutive_ints_usize(1, 2), 3);
        assert_eq!(sum_consecutive_ints_usize(1, 3), 6);
        assert_eq!(sum_consecutive_ints_usize(2, 1), 2);
        assert_eq!(sum_consecutive_ints_usize(2, 2), 5);
        assert_eq!(sum_consecutive_ints_usize(2, 3), 9);
    }
}
