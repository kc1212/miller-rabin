#![crate_type = "lib"]
#![crate_name = "miller_rabin"]

extern crate rand;
extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;

use num_bigint::{BigUint, ToBigUint, RandBigInt};
use num_traits::{Zero, One};
use num_integer::Integer;

// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Algorithm_and_running_time
pub fn probably_prime(n: &BigUint, k: u64) -> bool {
    let f1: BigUint = One::one();
    let f2: BigUint = 2.to_biguint().unwrap();
    let f3: BigUint = 3.to_biguint().unwrap();

    if n <= &f3 {
        if n == &f2 || n == &f3 {
            return true
        }
        return false
    }

    if n.is_even() {
        return false
    }

    let mut d = n - &f1;
    let mut r: u64 = 0;
    while d.is_even() {
        d = d >> 1;
        r += 1;
    }
    // assert_eq!(n - 1, 2u64.pow(r) * d);

    'witness: for _ in 0 .. k {
        let a = rand::thread_rng().gen_biguint_range(&f2, &(n - &f1));
        let mut x = mod_exp(&a, &d, n);
        if x == f1 || x == n - &f1 {
            continue
        }
        for _ in 0 .. r - 1 {
            x = mod_exp(&x, &f2, n);
            if x == f1 {
                return false
            }
            if x == n - &f1 {
                continue 'witness
            }
        }
        return false
    }
    true
}

// squared and multiply
pub fn mod_exp(b: &BigUint, e: &BigUint, m: &BigUint) -> BigUint {
    let (f1, f2): (BigUint, BigUint) = (One::one(), 2.to_biguint().unwrap());
    let mut res = f1.clone();
    let mut b_mut = b.clone();
    let mut e_mut = e.clone();

    while !e_mut.is_zero() {
        if e_mut.is_odd() {
            res = (res * &b_mut).mod_floor(m);
            e_mut = e_mut - &f1;
        }
        b_mut = (&b_mut * &b_mut).mod_floor(m);
        e_mut = e_mut / &f2;
    }

    res
}
