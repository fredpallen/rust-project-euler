pub mod p0001;
pub mod p0002;
pub mod p0003;
pub mod p0004;
pub mod p0005;
pub mod p0006;
pub mod p0007;
pub mod p0008;
pub mod p0009;
pub mod p0010;
pub mod p0011;
pub mod p0012;
pub mod p0013;
pub mod p0014;
pub mod p0015;
pub mod p0016;
pub mod p0017;
pub mod p0018;
pub mod p0019;
pub mod p0020;
pub mod p0021;
pub mod p0022;
pub mod p0023;
pub mod p0024;
pub mod p0025;
pub mod p0026;
pub mod p0027;
pub mod p0028;

use std::cmp;

pub fn gcd(a: u64, b: u64) -> u64 {
    let mut big = cmp::max(a, b);
    let mut small = cmp::min(a, b);
    if small == 0 {
        return 0;
    }
    while small != 0 {
        let r = big % small;
        big = small;
        small = r;
    }
    big
}

pub fn reverse_digits(n: u64) -> u64 {
    let mut remainder = n;
    let mut reversed = 0;
    while remainder != 0 {
        reversed *= 10;
        reversed += remainder % 10;
        remainder /= 10;
    }
    reversed
}

pub fn primes_to(n: usize) -> Vec<usize> {
    if n < 2 {return vec![];}
    if n < 3 {return vec![2];}

    let mut is_known_composite = vec![false; n + 1];
    let mut primes = vec![2, 3];

    let mut prime = 3;
    'main_loop: loop {
        let mut c = prime * prime;
        while c <= n {
            is_known_composite[c] = true;
            c += 2 * prime;
        }
        prime += 2;
        if prime > n {break 'main_loop; }
        while is_known_composite[prime] {
            prime += 2;
            if prime > n {break 'main_loop;}
        }
        primes.push(prime);
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(0, gcd(0, 0));
        assert_eq!(0, gcd(1, 0));
        assert_eq!(0, gcd(0, 1));
        assert_eq!(1, gcd(1, 1));
        assert_eq!(1, gcd(2, 1));
        assert_eq!(1, gcd(1, 2));
        assert_eq!(2, gcd(2, 2));
        assert_eq!(1, gcd(2, 3));
        assert_eq!(1, gcd(3, 2));
        assert_eq!(2, gcd(2, 4));
        assert_eq!(2, gcd(4, 2));
        assert_eq!(2, gcd(4, 6));
        assert_eq!(2, gcd(6, 4));
        assert_eq!(3, gcd(6, 9));
        assert_eq!(3, gcd(9, 6));
    }

    #[test]
    fn test_reverse_digits() {
        assert_eq!(0, reverse_digits(0));
        assert_eq!(1, reverse_digits(1));
        assert_eq!(9, reverse_digits(9));
        assert_eq!(1, reverse_digits(10));
        assert_eq!(11, reverse_digits(11));
        assert_eq!(21, reverse_digits(12));
        assert_eq!(1, reverse_digits(100));
        assert_eq!(101, reverse_digits(101));
        assert_eq!(201, reverse_digits(102));
    }

    #[test]
    fn test_primes_to() {
        assert!(primes_to(0).is_empty());
        assert!(primes_to(1).is_empty());
        assert_eq!(vec![2], primes_to(2));
        assert_eq!(vec![2,3], primes_to(3));
        assert_eq!(vec![2,3], primes_to(4));
        assert_eq!(vec![2,3,5], primes_to(5));
        assert_eq!(vec![2,3,5], primes_to(6));
        assert_eq!(vec![2,3,5,7], primes_to(7));
        assert_eq!(vec![2,3,5,7], primes_to(8));
        assert_eq!(vec![2,3,5,7], primes_to(9));
        assert_eq!(vec![2,3,5,7], primes_to(10));
        assert_eq!(vec![2,3,5,7,11], primes_to(11));
        assert_eq!(vec![2,3,5,7,11], primes_to(12));
        assert_eq!(vec![2,3,5,7,11,13], primes_to(13));
    }
}
