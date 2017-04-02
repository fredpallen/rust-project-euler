extern crate num;

use num::bigint::ToBigUint;
use num::BigUint;
use num::One;
use num::Zero;

trait SafeToBigUint: ToBigUint {
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

fn sum_consecutive_ints_usize(first: usize, count: usize) -> usize {
    if count == 0 {return 0;}

    // Sum the integers from 1 to count, inclusive.
    let sum_to = |count| count * (count + 1) / 2;

    let below_first = if first == 0 {0} else {sum_to(first - 1)};
    let to_last = sum_to(first + count - 1);
    to_last - below_first
}

fn sum_first_ints<T>(count: T) -> BigUint
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

fn sum_consecutive_ints<T, U>(first: T, count: U) -> BigUint
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

fn main() {
    println!("Hello, world!");
    println!(
        "sum_consecutive_ints(0, 3) = {}", sum_consecutive_ints(0u32, 3u32));
    println!(
        "sum_consecutive_ints_usize(0, 3) = {}", sum_consecutive_ints_usize(0usize, 3usize));
}
